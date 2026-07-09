"""
GFMSS Video Frame Interpolation Inference
Adapted for SlipperyEngine pipeline - processes frame directories
"""

import os
import sys
import cv2
import torch
import argparse
import numpy as np
from torch.nn import functional as F
from tqdm import tqdm
import warnings
import _thread
from queue import Queue, Empty

warnings.filterwarnings("ignore")

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, SCRIPT_DIR)

from model.GMFSS_infer_b import Model as ModelB
from model.GMFSS_infer_u import Model as ModelU

def get_cuda_version():
    if not torch.cuda.is_available():
        return None, None

    cuda_version = torch.version.cuda
    if cuda_version:
        major = cuda_version.split('.')[0]
        if major == '12':
            return '12x', 'cuda12x'
        elif major == '11':
            return '11x', 'cuda11x'
    return '12x', 'cuda12x'

def auto_gpu_setup():
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    if torch.cuda.is_available():
        gpu_name = torch.cuda.get_device_name(0)
        cuda_ver = torch.version.cuda or "unknown"
        print(f"[GPU] NVIDIA {gpu_name} detected (CUDA {cuda_ver})")

        cuda_variant, package_suffix = get_cuda_version()
        print(f"[GPU] Using CuPy package: cupy-{package_suffix}")

        torch.backends.cudnn.enabled = True
        torch.backends.cudnn.benchmark = True
        print(f"[GPU] cuDNN benchmark enabled")
    else:
        print("[GPU] No CUDA GPU detected, using CPU mode")
        print("[GPU] For GPU acceleration, ensure NVIDIA drivers and CUDA toolkit are installed")

    return device

parser = argparse.ArgumentParser(description='GFMSS Interpolation for frame directories')
parser.add_argument('--video', dest='video', type=str, default=None, help='Video file input (legacy, for compatibility)')
parser.add_argument('--frames', dest='frames', type=str, required=True, help='Directory containing input frames')
parser.add_argument('--output', dest='output', type=str, required=True, help='Output directory for interpolated frames')
parser.add_argument('--model', dest='model_dir', type=str, default=None, help='Model directory (defaults to train_log subdirectory)')
parser.add_argument('--fp16', dest='fp16', action='store_true', help='FP16 mode for faster inference on Tensor Cores')
parser.add_argument('--scale', dest='scale', type=float, default=1.0, help='Scale factor (0.25, 0.5, 1.0, 2.0, 4.0)')
parser.add_argument('--fps', dest='fps', type=float, default=None, help='Target FPS for naming')
parser.add_argument('--multi', dest='multi', type=int, default=2, help='Number of interpolation steps per frame pair')
parser.add_argument('--union', dest='union', action='store_true', help='Use union model (GMFSS_infer_u)')
parser.add_argument('--gpu', dest='gpu', type=str, default="auto", help='GPU selection (auto, cuda, cpu)')

args = parser.parse_args()

if args.scale not in [0.25, 0.5, 1.0, 2.0, 4.0]:
    print(f"Error: Scale must be one of [0.25, 0.5, 1.0, 2.0, 4.0], got {args.scale}")
    sys.exit(1)

device = auto_gpu_setup()
torch.set_grad_enabled(False)

if args.gpu == "cpu":
    device = torch.device("cpu")
    print("[GPU] Force CPU mode enabled")
elif args.gpu == "cuda" and not torch.cuda.is_available():
    print("[GPU] Warning: CUDA requested but not available, falling back to CPU")
    device = torch.device("cpu")

if device.type == "cuda" and args.fp16:
    torch.set_default_tensor_type(torch.cuda.HalfTensor)
    print("[GPU] FP16 mode enabled for Tensor Cores")

# Determine model directory - use train_log subdirectory
if args.model_dir is None:
    model_dir = os.path.join(SCRIPT_DIR, "train_log")
else:
    model_dir = args.model_dir

# Check if model weights exist
required_weights = ['flownet.pkl', 'metric.pkl', 'feat.pkl', 'fusionnet.pkl']
if args.union:
    required_weights.append('rife.pkl')

for weight in required_weights:
    weight_path = os.path.join(model_dir, weight)
    if not os.path.exists(weight_path):
        print(f"Error: Required weight file not found: {weight_path}")
        sys.exit(1)

# Initialize model based on union flag
print(f"Loading {'union' if args.union else 'base'} model from {model_dir}")
if args.union:
    model = ModelU()
else:
    model = ModelB()

model.version = 3.9  # Ensure version compatibility
model.load_model(model_dir, -1)
model.eval()
model.device()

print("Model loaded successfully")

# Get list of input frames
frames_dir = args.frames
frame_files = [f for f in os.listdir(frames_dir) if f.lower().endswith(('.png', '.jpg', '.jpeg', '.bmp'))]
frame_files.sort(key=lambda x: int(os.path.splitext(x)[0].split('_')[-1]))

if len(frame_files) < 2:
    print(f"Error: Need at least 2 frames, found {len(frame_files)}")
    sys.exit(1)

print(f"Processing {len(frame_files)} frames from {frames_dir}")

# Create output directory
os.makedirs(args.output, exist_ok=True)

# Setup padding for model
first_frame = cv2.imread(os.path.join(frames_dir, frame_files[0]), cv2.IMREAD_UNCHANGED)
if first_frame is None:
    print(f"Error: Could not read first frame: {frame_files[0]}")
    sys.exit(1)

h, w = first_frame.shape[:2]
tmp = max(64, int(64 / args.scale))
ph = ((h - 1) // tmp + 1) * tmp
pw = ((w - 1) // tmp + 1) * tmp
padding = (0, pw - w, 0, ph - h)

# Thread-safe buffers
write_buffer = Queue(maxsize=500)
read_buffer = Queue(maxsize=500)

def clear_write_buffer(user_args, write_buffer):
    """Thread function to write frames to output directory"""
    frame_idx = 0
    while True:
        item = write_buffer.get()
        if item is None:
            break
        output_path = os.path.join(user_args.output, f"{frame_idx:07d}.png")
        cv2.imwrite(output_path, item)
        frame_idx += 1

def build_read_buffer(user_args, read_buffer, frames_list):
    """Thread function to load frames into buffer"""
    for frame_file in frames_list:
        frame_path = os.path.join(user_args.frames, frame_file)
        frame = cv2.imread(frame_path, cv2.IMREAD_UNCHANGED)
        if frame is not None:
            read_buffer.put(frame)
    read_buffer.put(None)

def pad_image(img):
    """Pad image to model's required size"""
    if args.fp16:
        return F.pad(img, padding).half()
    else:
        return F.pad(img, padding)

def make_inference(I0, I1, reuse_things, n):
    """Generate n intermediate frames between I0 and I1"""
    if model.version >= 3.9:
        res = []
        for i in range(n):
            res.append(model.inference(I0, I1, reuse_things, (i + 1) * 1. / (n + 1)))
        return res
    else:
        middle = model.inference(I0, I1, args.scale)
        if n == 1:
            return [middle]
        first_half = make_inference(I0, middle, n=n // 2)
        second_half = make_inference(middle, I1, n=n // 2)
        if n % 2:
            return [*first_half, middle, *second_half]
        else:
            return [*first_half, *second_half]

# Start processing threads
_thread.start_new_thread(build_read_buffer, (args, read_buffer, frame_files))
_thread.start_new_thread(clear_write_buffer, (args, write_buffer))

# Process first frame
first_frame_rgb = first_frame[:, :, ::-1].copy()  # BGR to RGB
I1 = torch.from_numpy(np.transpose(first_frame_rgb, (2, 0, 1))).to(device, non_blocking=True).unsqueeze(0).float() / 255.
I1 = F.interpolate(I1, (ph, pw), mode='bilinear', align_corners=False)
temp = None

pbar = tqdm(total=len(frame_files) - 1, desc="Interpolating")
lastframe = first_frame

while True:
    if temp is not None:
        frame = temp
        temp = None
    else:
        frame = read_buffer.get()

    if frame is None:
        break

    # Prepare current frame
    I0 = I1
    frame_rgb = frame[:, :, ::-1].copy()  # BGR to RGB
    I1 = torch.from_numpy(np.transpose(frame_rgb, (2, 0, 1))).to(device, non_blocking=True).unsqueeze(0).float() / 255.
    I1 = F.interpolate(I1, (ph, pw), mode='bilinear', align_corners=False)

    # Run inference
    reuse_things = model.reuse(I0, I1, args.scale)
    output = make_inference(I0, I1, reuse_things, args.multi - 1)

    # Write original frame
    write_buffer.put(lastframe)

    # Write interpolated frames
    for mid in output:
        mid = F.interpolate(mid, (h, w), mode='bilinear', align_corners=False)
        mid = (((mid[0] * 255.).byte().cpu().numpy().transpose(1, 2, 0)))
        write_buffer.put(mid)

    pbar.update(1)
    lastframe = frame

# Write the last frame
write_buffer.put(lastframe)

# Wait for all frames to be written
while not write_buffer.empty():
    import time
    time.sleep(0.1)

pbar.close()

# Signal write thread to finish
write_buffer.put(None)

print(f"Interpolation complete. Output saved to: {args.output}")
print(f"Total output frames: {len([f for f in os.listdir(args.output) if f.endswith('.png')])}")
