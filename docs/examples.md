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
    
    return {
        'count': len(data),
        'sum': ao.sum(data),
        'mean': ao.mean(data),
        'min': ao.min(data),
        'max': ao.max(data),
        'std': ao.std(data),
        'var': ao.var(data),
        'median': ao.median(data),
    }

data = array.array('f', [10.0, 20.0, 30.0, 40.0, 50.0])
stats = compute_stats(data)
print(stats)
# {'count': 5, 'sum': 150.0, 'mean': 30.0, 'min': 10.0, 'max': 50.0,
#  'std': 14.14..., 'var': 200.0, 'median': 30.0}
```

### Data Transformation Pipeline

```python
import array
import arrayops as ao

def transform_pipeline(data):
    """Apply a series of transformations."""
    # Step 1: Center the data (subtract mean)
    mean_val = ao.mean(data)
    for i in range(len(data)):
        data[i] -= mean_val
    
    # Step 2: Scale to unit variance
    std_dev = ao.std(data)
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
# Create a copy for normalization (since normalize modifies in-place)
normalized = array.array('f', list(valid))
ao.normalize(normalized)
print(f"Normalized: {list(normalized)}")  # Values scaled to 0-1 range

# Step 3: Compute statistics
mean_val = ao.mean(normalized)
print(f"Mean normalized value: {mean_val:.3f}")
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

## Statistical Operations

### Basic Statistics

```python
import array
import arrayops as ao

data = array.array('i', [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

# Basic statistics
print(f"Mean: {ao.mean(data)}")          # 5.5
print(f"Min: {ao.min(data)}")            # 1
print(f"Max: {ao.max(data)}")            # 10
print(f"Std dev: {ao.std(data):.2f}")    # 2.87
print(f"Variance: {ao.var(data):.2f}")   # 8.25
print(f"Median: {ao.median(data)}")      # 5
```

### Data Analysis

```python
import array
import arrayops as ao

# Analyze temperature readings
temperatures = array.array('f', [20.5, 22.1, 19.8, 21.3, 23.0, 20.2, 21.7])

stats = {
    'mean': ao.mean(temperatures),
    'min': ao.min(temperatures),
    'max': ao.max(temperatures),
    'std': ao.std(temperatures),
    'median': ao.median(temperatures),
}

print(f"Temperature stats: {stats}")
# Temperature stats: {'mean': 21.23..., 'min': 19.8, 'max': 23.0,
#                     'std': 1.03..., 'median': 21.3}
```

---

## Element-wise Operations

### Array Addition

```python
import array
import arrayops as ao

# Add two arrays element-wise
arr1 = array.array('i', [1, 2, 3, 4, 5])
arr2 = array.array('i', [10, 20, 30, 40, 50])
result = ao.add(arr1, arr2)
print(list(result))  # [11, 22, 33, 44, 55]

# Vector addition for coordinates
x_coords = array.array('f', [1.0, 2.0, 3.0])
y_coords = array.array('f', [4.0, 5.0, 6.0])
translated = ao.add(x_coords, y_coords)
print(list(translated))  # [5.0, 7.0, 9.0]
```

### Array Multiplication

```python
import array
import arrayops as ao

# Element-wise multiplication
arr1 = array.array('i', [1, 2, 3, 4, 5])
arr2 = array.array('i', [2, 3, 4, 5, 6])
result = ao.multiply(arr1, arr2)
print(list(result))  # [2, 6, 12, 20, 30]

# Apply scaling factors
values = array.array('f', [10.0, 20.0, 30.0])
scales = array.array('f', [0.5, 1.5, 2.0])
scaled = ao.multiply(values, scales)
print(list(scaled))  # [5.0, 30.0, 60.0]
```

### Clipping Values

```python
import array
import arrayops as ao

# Clip values to valid range
data = array.array('i', [1, 5, 10, 15, 20, 25])
ao.clip(data, 5.0, 15.0)
print(list(data))  # [5, 5, 10, 15, 15, 15]

# Ensure sensor readings are in valid range
readings = array.array('f', [-5.0, 10.0, 25.0, 35.0, 50.0])
ao.clip(readings, 0.0, 40.0)
print(list(readings))  # [0.0, 10.0, 25.0, 35.0, 40.0]
```

### Normalization

```python
import array
import arrayops as ao

# Normalize data to [0, 1] range
data = array.array('f', [10.0, 20.0, 30.0, 40.0, 50.0])
ao.normalize(data)
print(list(data))  # [0.0, 0.25, 0.5, 0.75, 1.0]

# Normalize feature vectors for machine learning
features = array.array('f', [100.0, 200.0, 300.0, 400.0])
ao.normalize(features)
print(list(features))  # [0.0, 0.333..., 0.666..., 1.0]
```

---

## Array Manipulation

### Reversing Arrays

```python
import array
import arrayops as ao

# Reverse array in-place
arr = array.array('i', [1, 2, 3, 4, 5])
ao.reverse(arr)
print(list(arr))  # [5, 4, 3, 2, 1]

# Reverse time series data
timeline = array.array('f', [1.0, 2.0, 3.0, 4.0, 5.0])
ao.reverse(timeline)
print(list(timeline))  # [5.0, 4.0, 3.0, 2.0, 1.0]
```

### Sorting Arrays

```python
import array
import arrayops as ao

# Sort array in-place
arr = array.array('i', [5, 2, 8, 1, 9, 3])
ao.sort(arr)
print(list(arr))  # [1, 2, 3, 5, 8, 9]

# Sort floating point data
values = array.array('f', [3.5, 1.2, 7.8, 0.5, 4.9])
ao.sort(values)
print(list(values))  # [0.5, 1.2, 3.5, 4.9, 7.8]
```

### Finding Unique Values

```python
import array
import arrayops as ao

# Get unique elements (sorted)
arr = array.array('i', [5, 2, 8, 2, 1, 5, 9, 1])
unique_vals = ao.unique(arr)
print(list(unique_vals))  # [1, 2, 5, 8, 9]

# Remove duplicates from category IDs
categories = array.array('i', [1, 2, 2, 3, 1, 3, 4, 2])
unique_categories = ao.unique(categories)
print(list(unique_categories))  # [1, 2, 3, 4]
```

### Combining Operations

```python
import array
import arrayops as ao

# Data cleaning pipeline
data = array.array('f', [10.0, 25.0, 15.0, 30.0, 20.0, 35.0, 5.0])

# Step 1: Sort data
ao.sort(data)

# Step 2: Clip outliers
ao.clip(data, 10.0, 30.0)

# Step 3: Normalize
ao.normalize(data)

# Step 4: Get unique normalized values
unique_normalized = ao.unique(data)
print(list(unique_normalized))
```

---

## Zero-Copy Slicing

### Basic Slicing

```python
import array
import arrayops as ao

# Create array and slice it
arr = array.array('i', [10, 20, 30, 40, 50, 60])
view = ao.slice(arr, 1, 4)
print(list(view))  # [20, 30, 40]
print(type(view))  # <class 'memoryview'>

# Slice with defaults
start_slice = ao.slice(arr, None, 3)  # [10, 20, 30]
end_slice = ao.slice(arr, 3, None)    # [40, 50, 60]
full_slice = ao.slice(arr, None, None)  # [10, 20, 30, 40, 50, 60]
```

### Zero-Copy Behavior

```python
import array
import arrayops as ao

# Create array and view
arr = array.array('i', [1, 2, 3, 4, 5])
view = ao.slice(arr, 1, 4)  # View: [2, 3, 4]

# Modify original array
arr[2] = 99

# View reflects the change (zero-copy)
print(list(view))  # [2, 99, 4]
```

### Processing Chunks

```python
import array
import arrayops as ao

# Process array in chunks without copying
data = array.array('f', range(100))
chunk_size = 20

for i in range(0, len(data), chunk_size):
    chunk = ao.slice(data, i, i + chunk_size)
    # Process chunk (zero-copy view)
    avg = ao.mean(chunk)  # Can use arrayops functions directly on memoryview
    print(f"Chunk {i//chunk_size} average: {avg}")
```

---

## Lazy Evaluation

### Basic Lazy Operations

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

# Create lazy array and chain operations
lazy = ao.lazy_array(arr)
result = lazy.map(lambda x: x * 2).filter(lambda x: x > 10).collect()
print(list(result))  # [12, 14, 16, 18, 20]
```

### Chaining Multiple Operations

```python
import array
import arrayops as ao

# Chain map and filter operations efficiently
data = array.array('i', [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

lazy = ao.lazy_array(data)
# Multiple maps
result = lazy.map(lambda x: x * 2).map(lambda x: x + 1).collect()
print(list(result))  # [3, 5, 7, 9, 11, 13, 15, 17, 19, 21]

# Multiple filters
lazy = ao.lazy_array(data)
result = lazy.filter(lambda x: x > 3).filter(lambda x: x < 8).collect()
print(list(result))  # [4, 5, 6, 7]
```

### Memory Efficiency

```python
import array
import arrayops as ao

# Without lazy evaluation: creates intermediate arrays
data = array.array('i', range(100000))
doubled = ao.map(data, lambda x: x * 2)
filtered = ao.filter(doubled, lambda x: x > 1000)
result = ao.sum(filtered)  # Multiple intermediate arrays allocated

# With lazy evaluation: single allocation
data = array.array('i', range(100000))
lazy = ao.lazy_array(data)
filtered = lazy.map(lambda x: x * 2).filter(lambda x: x > 1000).collect()
result = ao.sum(filtered)  # Only one intermediate array allocated
```

### Checking Chain Length

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
lazy = ao.lazy_array(arr)

lazy = lazy.map(lambda x: x * 2)
print(lazy.len())  # 1

lazy = lazy.filter(lambda x: x > 5)
print(lazy.len())  # 2

lazy = lazy.map(lambda x: x + 1)
print(lazy.len())  # 3
```

---

## Iterator Protocol

### Basic Iteration

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Create an iterator
it = ao.array_iterator(arr)

# Use in a for loop
for value in it:
    print(value)  # Prints: 1, 2, 3, 4, 5

# Convert to list
it = ao.array_iterator(arr)
values = list(it)
print(values)  # [1, 2, 3, 4, 5]
```

### Using with Built-in Functions

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Use with sum()
it = ao.array_iterator(arr)
total = sum(it)
print(total)  # 15

# Use with any() and all()
it = ao.array_iterator(arr)
has_positive = any(x > 0 for x in it)  # True

it = ao.array_iterator(arr)
all_positive = all(x > 0 for x in it)  # True
```

### List Comprehensions

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# Use in list comprehension
it = ao.array_iterator(arr)
doubled = [x * 2 for x in it]
print(doubled)  # [2, 4, 6, 8, 10]

# Filter while iterating
it = ao.array_iterator(arr)
evens = [x for x in it if x % 2 == 0]
print(evens)  # [2, 4]
```

### Using next() Function

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])
it = ao.array_iterator(arr)

# Get individual elements
first = next(it)  # 1
second = next(it)  # 2
third = next(it)  # 3

# StopIteration raised when exhausted
try:
    while True:
        value = next(it)
        print(value)
except StopIteration:
    print("Iterator exhausted")
```

### Iterating LazyArray

```python
import array
import arrayops as ao

arr = array.array('i', [1, 2, 3, 4, 5])

# LazyArray supports iteration
lazy = ao.lazy_array(arr)
lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)

# Iterate directly - chain is evaluated automatically
for value in lazy:
    print(value)  # Prints: 6, 8, 10

# Can also convert to list
result = list(lazy)
print(result)  # [6, 8, 10]
```

### Different Array Types

```python
import array
import arrayops as ao

# Works with array.array
arr = array.array('i', [1, 2, 3])
it = ao.array_iterator(arr)
print(list(it))  # [1, 2, 3]

# Works with numpy arrays
import numpy as np
narr = np.array([1.5, 2.5, 3.5], dtype=np.float32)
it = ao.array_iterator(narr)
print(list(it))  # [1.5, 2.5, 3.5]

# Works with memoryview
mv = memoryview(arr)
it = ao.array_iterator(mv)
print(list(it))  # [1, 2, 3]
```

---

## Best Practices

1. **Use appropriate types**: Choose the smallest numeric type that fits your data to save memory
2. **Prefer in-place operations**: Use `map_inplace()`, `scale()`, `clip()`, `normalize()`, `reverse()`, and `sort()` which modify arrays in-place when possible
3. **Handle empty arrays**: Always check for empty arrays before operations if needed (statistical operations raise errors on empty arrays)
4. **Batch processing**: Process large datasets in chunks to manage memory
5. **Type consistency**: Keep arrays of the same type throughout your pipeline
6. **Use map_inplace for memory efficiency**: When you don't need the original array, use `map_inplace()` instead of `map()`
7. **Combine operations**: Chain map, filter, reduce, and array manipulation operations for complex data processing pipelines
8. **Use statistical functions**: Prefer `ao.mean()`, `ao.min()`, `ao.max()`, etc. over computing manually for better performance
9. **Normalize before ML**: Use `ao.normalize()` to scale features to [0, 1] range before machine learning operations
10. **Unique for deduplication**: Use `ao.unique()` to efficiently remove duplicates and get sorted unique values
11. **Use zero-copy slicing**: Use `ao.slice()` for views of array portions without copying data
12. **Lazy evaluation for chains**: Use `ao.lazy_array()` to chain multiple `map()` and `filter()` operations efficiently, avoiding intermediate allocations
13. **Efficient iteration**: Use `ao.array_iterator()` for Rust-optimized iteration over arrays when you need to iterate element-by-element

## Related Documentation

- [API Reference](api.md) - Complete function documentation
- [Performance Guide](performance.md) - Performance optimization
- [Troubleshooting](troubleshooting.md) - Common issues

