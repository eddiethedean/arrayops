# Examples and Cookbook

Practical examples and real-world use cases for `arrayops`.

## Basic Operations

### Summing Arrays

```python
import array
import arrayops as ao

# Simple sum
data = array.array('i', [10, 20, 30, 40, 50])
total = ao.sum(data)
print(f"Sum: {total}")  # Sum: 150

# Float arrays
temperatures = array.array('f', [20.5, 21.3, 19.8, 22.1])
avg_temp = ao.sum(temperatures) / len(temperatures)
print(f"Average temperature: {avg_temp:.2f}°C")
```

### Scaling Arrays

```python
import array
import arrayops as ao

# Scale by a factor
data = array.array('i', [1, 2, 3, 4, 5])
ao.scale(data, 2.0)
print(list(data))  # [2, 4, 6, 8, 10]

# Normalize to percentage
values = array.array('f', [25.0, 50.0, 75.0, 100.0])
ao.scale(values, 0.01)  # Convert to 0.0-1.0 range
print(list(values))  # [0.25, 0.5, 0.75, 1.0]
```

### Mapping Arrays

```python
import array
import arrayops as ao

# Double each element
data = array.array('i', [1, 2, 3, 4, 5])
doubled = ao.map(data, lambda x: x * 2)
print(list(doubled))  # [2, 4, 6, 8, 10]

# Square values
squared = ao.map(data, lambda x: x * x)
print(list(squared))  # [1, 4, 9, 16, 25]

# In-place transformation (more efficient)
ao.map_inplace(data, lambda x: x * 2)
print(list(data))  # [2, 4, 6, 8, 10]
```

### Filtering Arrays

```python
import array
import arrayops as ao

# Filter even numbers
data = array.array('i', [1, 2, 3, 4, 5, 6])
evens = ao.filter(data, lambda x: x % 2 == 0)
print(list(evens))  # [2, 4, 6]

# Filter values above threshold
large = ao.filter(data, lambda x: x > 3)
print(list(large))  # [4, 5, 6]
```

### Reducing Arrays

```python
import array
import arrayops as ao

data = array.array('i', [1, 2, 3, 4, 5])

# Sum
total = ao.reduce(data, lambda acc, x: acc + x)
print(total)  # 15

# Product
product = ao.reduce(data, lambda acc, x: acc * x, initial=1)
print(product)  # 120
```

## Binary Protocol Parsing

### Reading Sensor Data

```python
import array
import arrayops as ao

# Read binary sensor data from file
with open('sensor_data.bin', 'rb') as f:
    data = array.array('f')  # float32
    data.fromfile(f, 10000)  # Read 10,000 floats

# Fast aggregation
total = ao.sum(data)
mean = total / len(data)
print(f"Average reading: {mean:.2f}")

# Scale readings to different units
# Convert from Celsius to Fahrenheit
ao.scale(data, 9.0 / 5.0)  # Multiply by 9/5
# Note: You'd need to add 32 for full conversion, but scale handles multiplication
```

### Network Packet Analysis

```python
import array
import arrayops as ao
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
total_bytes = ao.sum(packet_sizes)
avg_packet_size = total_bytes / len(packet_sizes)
print(f"Total bytes: {total_bytes}")
print(f"Average packet size: {avg_packet_size:.2f} bytes")
```

## ETL Pipeline

### Data Normalization

```python
import array
import arrayops as ao

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
    ao.scale(sensor_readings, 1.0 / range_size)

print(list(sensor_readings))  # All values now in [0, 1] range
```

### Batch Processing

```python
import array
import arrayops as ao

def process_batch(readings):
    """Process a batch of sensor readings."""
    # Compute statistics
    total = ao.sum(readings)
    mean = total / len(readings)
    
    # Normalize batch
    min_val = min(readings)
    max_val = max(readings)
    if max_val > min_val:
        for i in range(len(readings)):
            readings[i] -= min_val
        ao.scale(readings, 1.0 / (max_val - min_val))
    
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
import arrayops as ao

# Read grayscale image data (8-bit pixels)
with open('image.raw', 'rb') as f:
    pixels = array.array('B')  # uint8
    pixels.fromfile(f, 1024 * 1024)  # 1MP image

# Compute average brightness
total_brightness = ao.sum(pixels)
avg_brightness = total_brightness / len(pixels)
print(f"Average brightness: {avg_brightness:.2f}")

# Adjust brightness (scale by factor)
brightness_factor = 1.2  # Increase by 20%
ao.scale(pixels, brightness_factor)
# Note: Values will be clamped by uint8 type
```

### Processing Audio Samples

```python
import array
import arrayops as ao

# Read 16-bit audio samples
with open('audio.raw', 'rb') as f:
    samples = array.array('h')  # int16
    samples.fromfile(f, 44100)  # 1 second at 44.1kHz

# Compute average amplitude
total = ao.sum(samples)
mean = total / len(samples)

# Normalize audio (scale to use full dynamic range)
max_amplitude = max(abs(s) for s in samples)
if max_amplitude > 0:
    # Scale to use 80% of max range to avoid clipping
    scale_factor = (32767 * 0.8) / max_amplitude
    ao.scale(samples, scale_factor)
```

## Performance Optimization Tips

### Prefer In-Place Operations

```python
import array
import arrayops as ao

# Good: In-place scaling (no allocation)
data = array.array('i', [1, 2, 3, 4, 5])
ao.scale(data, 2.0)  # Modifies existing array

# Avoid: Creating new arrays unnecessarily
# (When future operations support it, prefer in-place)
```

### Batch Processing

```python
import array
import arrayops as ao

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
            total = ao.sum(batch)
            ao.scale(batch, 0.001)  # Normalize
            results.append(total)
    
    return results
```

### Type Selection

```python
import array
import arrayops as ao

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
import arrayops as ao

def compute_stats(data):
    """Compute basic statistics for an array."""
    if len(data) == 0:
        return None
    
    total = ao.sum(data)
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
import arrayops as ao

def transform_pipeline(data):
    """Apply a series of transformations."""
    # Step 1: Center the data (subtract mean)
    mean = ao.sum(data) / len(data)
    for i in range(len(data)):
        data[i] -= mean
    
    # Step 2: Scale to unit variance (simplified)
    # In practice, you'd compute variance first
    variance_estimate = 1.0  # Placeholder
    std_dev = variance_estimate ** 0.5
    if std_dev > 0:
        ao.scale(data, 1.0 / std_dev)
    
    return data

data = array.array('f', [1.0, 2.0, 3.0, 4.0, 5.0])
transformed = transform_pipeline(data)
print(list(transformed))
```

## Data Transformation

### Map Operations

```python
import array
import arrayops as ao

# Transform sensor readings
readings = array.array('f', [10.5, 20.3, 15.8, 30.2, 25.1])

# Convert Celsius to Fahrenheit
fahrenheit = ao.map(readings, lambda c: c * 9.0 / 5.0 + 32.0)
print(list(fahrenheit))  # [50.9, 68.54, 60.44, 86.36, 77.18]

# Square all values
squared = ao.map(readings, lambda x: x * x)
print(list(squared))  # [110.25, 412.09, 249.64, 912.04, 630.01]

# In-place transformation (more memory efficient)
ao.map_inplace(readings, lambda x: x * 2.0)
print(list(readings))  # [21.0, 40.6, 31.6, 60.4, 50.2]
```

### Data Filtering

```python
import array
import arrayops as ao

# Filter sensor readings
readings = array.array('f', [10.5, 20.3, 15.8, 30.2, 25.1, 5.2, 35.0])

# Filter readings above threshold
high_readings = ao.filter(readings, lambda x: x > 20.0)
print(list(high_readings))  # [20.3, 30.2, 25.1, 35.0]

# Filter even integers
numbers = array.array('i', [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
evens = ao.filter(numbers, lambda x: x % 2 == 0)
print(list(evens))  # [2, 4, 6, 8, 10]

# Filter valid ranges
temperatures = array.array('f', [-10.0, 20.0, 25.0, 30.0, 45.0, -5.0])
valid_temps = ao.filter(temperatures, lambda t: 0.0 <= t <= 40.0)
print(list(valid_temps))  # [20.0, 25.0, 30.0]
```

### Aggregation

```python
import array
import arrayops as ao

data = array.array('i', [1, 2, 3, 4, 5])

# Sum
total = ao.reduce(data, lambda acc, x: acc + x)
print(total)  # 15

# Product
product = ao.reduce(data, lambda acc, x: acc * x, initial=1)
print(product)  # 120

# Maximum
maximum = ao.reduce(data, lambda acc, x: acc if acc > x else x)
print(maximum)  # 5

# Minimum
minimum = ao.reduce(data, lambda acc, x: acc if acc < x else x)
print(minimum)  # 1

# Count elements (using reduce)
count = ao.reduce(data, lambda acc, x: acc + 1, initial=0)
print(count)  # 5
```

### Complex Pipelines

```python
import array
import arrayops as ao

# Process sensor data pipeline
sensor_data = array.array('f', [10.5, 20.3, 15.8, 30.2, 25.1, 5.2, 35.0])

# Step 1: Filter valid readings (0-40 range)
valid = ao.filter(sensor_data, lambda x: 0.0 <= x <= 40.0)
print(f"Valid readings: {list(valid)}")  # [10.5, 20.3, 15.8, 30.2, 25.1, 5.2, 35.0]

# Step 2: Transform (normalize to 0-1)
max_val = ao.reduce(valid, lambda acc, x: acc if acc > x else x)
normalized = ao.map(valid, lambda x: x / max_val)
print(f"Normalized: {list(normalized)}")  # Values scaled to 0-1 range

# Step 3: Compute statistics
total = ao.reduce(normalized, lambda acc, x: acc + x)
mean = total / len(normalized)
print(f"Mean normalized value: {mean:.3f}")
```

### Map-Filter-Reduce Pattern

```python
import array
import arrayops as ao

# Process transaction amounts
transactions = array.array('i', [100, 200, -50, 300, -25, 150, -100, 250])

# Map: Get absolute values
abs_transactions = ao.map(transactions, lambda x: abs(x))
print(list(abs_transactions))  # [100, 200, 50, 300, 25, 150, 100, 250]

# Filter: Only large transactions (>100)
large = ao.filter(abs_transactions, lambda x: x > 100)
print(list(large))  # [200, 300, 150, 250]

# Reduce: Sum of large transactions
total_large = ao.reduce(large, lambda acc, x: acc + x)
print(total_large)  # 900
```

## Best Practices

1. **Use appropriate types**: Choose the smallest numeric type that fits your data to save memory
2. **Prefer in-place operations**: Use `map_inplace()` and `scale()` which modify arrays in-place when possible
3. **Handle empty arrays**: Always check for empty arrays before operations if needed
4. **Batch processing**: Process large datasets in chunks to manage memory
5. **Type consistency**: Keep arrays of the same type throughout your pipeline
6. **Use map_inplace for memory efficiency**: When you don't need the original array, use `map_inplace()` instead of `map()`
7. **Combine operations**: Chain map, filter, and reduce operations for complex data processing pipelines

## Related Documentation

- [API Reference](api.md) - Complete function documentation
- [Performance Guide](performance.md) - Performance optimization
- [Troubleshooting](troubleshooting.md) - Common issues

