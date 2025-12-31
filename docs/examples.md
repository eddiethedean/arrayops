# Examples and Cookbook

Practical examples and real-world use cases for `arrayops`.

## Basic Operations

### Summing Arrays

```python
import array
import arrayops

# Simple sum
data = array.array('i', [10, 20, 30, 40, 50])
total = arrayops.sum(data)
print(f"Sum: {total}")  # Sum: 150

# Float arrays
temperatures = array.array('f', [20.5, 21.3, 19.8, 22.1])
avg_temp = arrayops.sum(temperatures) / len(temperatures)
print(f"Average temperature: {avg_temp:.2f}°C")
```

### Scaling Arrays

```python
import array
import arrayops

# Scale by a factor
data = array.array('i', [1, 2, 3, 4, 5])
arrayops.scale(data, 2.0)
print(list(data))  # [2, 4, 6, 8, 10]

# Normalize to percentage
values = array.array('f', [25.0, 50.0, 75.0, 100.0])
arrayops.scale(values, 0.01)  # Convert to 0.0-1.0 range
print(list(values))  # [0.25, 0.5, 0.75, 1.0]
```

## Binary Protocol Parsing

### Reading Sensor Data

```python
import array
import arrayops

# Read binary sensor data from file
with open('sensor_data.bin', 'rb') as f:
    data = array.array('f')  # float32
    data.fromfile(f, 10000)  # Read 10,000 floats

# Fast aggregation
total = arrayops.sum(data)
mean = total / len(data)
print(f"Average reading: {mean:.2f}")

# Scale readings to different units
# Convert from Celsius to Fahrenheit
arrayops.scale(data, 9.0 / 5.0)  # Multiply by 9/5
# Note: You'd need to add 32 for full conversion, but scale handles multiplication
```

### Network Packet Analysis

```python
import array
import arrayops
import struct

# Parse packet sizes from binary log
packet_sizes = array.array('I')  # uint32

# Read packet header sizes
with open('packets.bin', 'rb') as f:
    while True:
        chunk = f.read(4)
        if len(chunk) < 4:
            break
        size = struct.unpack('I', chunk)[0]
        packet_sizes.append(size)

# Analyze packet sizes
total_bytes = arrayops.sum(packet_sizes)
avg_packet_size = total_bytes / len(packet_sizes)
print(f"Total bytes: {total_bytes}")
print(f"Average packet size: {avg_packet_size:.2f} bytes")
```

## ETL Pipeline

### Data Normalization

```python
import array
import arrayops

# Load sensor readings
sensor_readings = array.array('f', [10.5, 25.3, 15.8, 30.2, 20.1])

# Normalize to 0-1 range
min_val = min(sensor_readings)
max_val = max(sensor_readings)
range_size = max_val - min_val

if range_size > 0:
    # Shift to start at 0
    for i in range(len(sensor_readings)):
        sensor_readings[i] -= min_val
    # Scale to 0-1
    arrayops.scale(sensor_readings, 1.0 / range_size)

print(list(sensor_readings))  # All values now in [0, 1] range
```

### Batch Processing

```python
import array
import arrayops

def process_batch(readings):
    """Process a batch of sensor readings."""
    # Compute statistics
    total = arrayops.sum(readings)
    mean = total / len(readings)
    
    # Normalize batch
    min_val = min(readings)
    max_val = max(readings)
    if max_val > min_val:
        for i in range(len(readings)):
            readings[i] -= min_val
        arrayops.scale(readings, 1.0 / (max_val - min_val))
    
    return mean, readings

# Process multiple batches
batches = [
    array.array('f', [10.0, 20.0, 30.0]),
    array.array('f', [15.0, 25.0, 35.0]),
    array.array('f', [12.0, 22.0, 32.0]),
]

for i, batch in enumerate(batches):
    mean, normalized = process_batch(batch)
    print(f"Batch {i}: mean={mean:.2f}, normalized={list(normalized)}")
```

## File Format Handling

### Reading Binary Image Data

```python
import array
import arrayops

# Read grayscale image data (8-bit pixels)
with open('image.raw', 'rb') as f:
    pixels = array.array('B')  # uint8
    pixels.fromfile(f, 1024 * 1024)  # 1MP image

# Compute average brightness
total_brightness = arrayops.sum(pixels)
avg_brightness = total_brightness / len(pixels)
print(f"Average brightness: {avg_brightness:.2f}")

# Adjust brightness (scale by factor)
brightness_factor = 1.2  # Increase by 20%
arrayops.scale(pixels, brightness_factor)
# Note: Values will be clamped by uint8 type
```

### Processing Audio Samples

```python
import array
import arrayops

# Read 16-bit audio samples
with open('audio.raw', 'rb') as f:
    samples = array.array('h')  # int16
    samples.fromfile(f, 44100)  # 1 second at 44.1kHz

# Compute average amplitude
total = arrayops.sum(samples)
mean = total / len(samples)

# Normalize audio (scale to use full dynamic range)
max_amplitude = max(abs(s) for s in samples)
if max_amplitude > 0:
    # Scale to use 80% of max range to avoid clipping
    scale_factor = (32767 * 0.8) / max_amplitude
    arrayops.scale(samples, scale_factor)
```

## Performance Optimization Tips

### Prefer In-Place Operations

```python
import array
import arrayops

# Good: In-place scaling (no allocation)
data = array.array('i', [1, 2, 3, 4, 5])
arrayops.scale(data, 2.0)  # Modifies existing array

# Avoid: Creating new arrays unnecessarily
# (When future operations support it, prefer in-place)
```

### Batch Processing

```python
import array
import arrayops

# Process large datasets in batches
def process_large_dataset(file_path, batch_size=10000):
    results = []
    with open(file_path, 'rb') as f:
        while True:
            batch = array.array('f')
            try:
                batch.fromfile(f, batch_size)
            except EOFError:
                break
            
            if len(batch) == 0:
                break
            
            # Process batch
            total = arrayops.sum(batch)
            arrayops.scale(batch, 0.001)  # Normalize
            results.append(total)
    
    return results
```

### Type Selection

```python
import array
import arrayops

# Choose appropriate types for your data
# Use smallest type that fits your data range

# For small integers (0-255)
small_values = array.array('B', [100, 150, 200])  # uint8

# For larger integers
large_values = array.array('i', [1000000, 2000000])  # int32

# For floating point
float_values = array.array('f', [1.5, 2.5, 3.5])  # float32
# Use 'd' (float64) only if you need double precision
```

## Common Patterns

### Computing Statistics

```python
import array
import arrayops

def compute_stats(data):
    """Compute basic statistics for an array."""
    if len(data) == 0:
        return None
    
    total = arrayops.sum(data)
    mean = total / len(data)
    
    # Find min/max (using built-in for now)
    min_val = min(data)
    max_val = max(data)
    
    return {
        'count': len(data),
        'sum': total,
        'mean': mean,
        'min': min_val,
        'max': max_val,
        'range': max_val - min_val,
    }

data = array.array('f', [10.0, 20.0, 30.0, 40.0, 50.0])
stats = compute_stats(data)
print(stats)
```

### Data Transformation Pipeline

```python
import array
import arrayops

def transform_pipeline(data):
    """Apply a series of transformations."""
    # Step 1: Center the data (subtract mean)
    mean = arrayops.sum(data) / len(data)
    for i in range(len(data)):
        data[i] -= mean
    
    # Step 2: Scale to unit variance (simplified)
    # In practice, you'd compute variance first
    variance_estimate = 1.0  # Placeholder
    std_dev = variance_estimate ** 0.5
    if std_dev > 0:
        arrayops.scale(data, 1.0 / std_dev)
    
    return data

data = array.array('f', [1.0, 2.0, 3.0, 4.0, 5.0])
transformed = transform_pipeline(data)
print(list(transformed))
```

## Best Practices

1. **Use appropriate types**: Choose the smallest numeric type that fits your data to save memory
2. **Prefer in-place operations**: Use `scale()` which modifies arrays in-place when possible
3. **Handle empty arrays**: Always check for empty arrays before operations if needed
4. **Batch processing**: Process large datasets in chunks to manage memory
5. **Type consistency**: Keep arrays of the same type throughout your pipeline

## Related Documentation

- [API Reference](api.md) - Complete function documentation
- [Performance Guide](performance.md) - Performance optimization
- [Troubleshooting](troubleshooting.md) - Common issues

