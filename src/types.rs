use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

/// Supported array.array typecodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeCode {
    // Signed integers
    Int8,  // 'b'
    Int16, // 'h'
    Int32, // 'i'
    Int64, // 'l'
    // Unsigned integers
    UInt8,  // 'B'
    UInt16, // 'H'
    UInt32, // 'I'
    UInt64, // 'L'
    // Floats
    Float32, // 'f'
    Float64, // 'd'
}

impl TypeCode {
    /// Parse typecode from Python array.array typecode
    pub fn from_char(typecode: char) -> PyResult<Self> {
        match typecode {
            'b' => Ok(TypeCode::Int8),
            'h' => Ok(TypeCode::Int16),
            'i' => Ok(TypeCode::Int32),
            'l' => Ok(TypeCode::Int64),
            'B' => Ok(TypeCode::UInt8),
            'H' => Ok(TypeCode::UInt16),
            'I' => Ok(TypeCode::UInt32),
            'L' => Ok(TypeCode::UInt64),
            'f' => Ok(TypeCode::Float32),
            'd' => Ok(TypeCode::Float64),
            _ => Err(PyTypeError::new_err(format!(
                "Unsupported typecode: '{typecode}'. Supported: b, B, h, H, i, I, l, L, f, d"
            ))),
        }
    }

    /// Get typecode as char
    pub fn as_char(&self) -> char {
        match self {
            TypeCode::Int8 => 'b',
            TypeCode::Int16 => 'h',
            TypeCode::Int32 => 'i',
            TypeCode::Int64 => 'l',
            TypeCode::UInt8 => 'B',
            TypeCode::UInt16 => 'H',
            TypeCode::UInt32 => 'I',
            TypeCode::UInt64 => 'L',
            TypeCode::Float32 => 'f',
            TypeCode::Float64 => 'd',
        }
    }
}

/// Get typecode from Python array.array object
pub(crate) fn get_typecode(array: &Bound<'_, PyAny>) -> PyResult<TypeCode> {
    let typecode_attr = array.getattr("typecode")?;
    let typecode_str_obj = typecode_attr.str()?;
    let typecode_str = typecode_str_obj.to_string_lossy();
    if typecode_str.len() != 1 {
        return Err(PyTypeError::new_err("Invalid typecode"));
    }
    TypeCode::from_char(typecode_str.chars().next().unwrap())
}

/// Get typecode from numpy.ndarray dtype
pub(crate) fn get_numpy_typecode(arr: &Bound<'_, PyAny>) -> PyResult<TypeCode> {
    let dtype = arr.getattr("dtype")?;
    let char_attr = dtype.getattr("char")?;
    let dtype_str_obj = char_attr.str()?;
    let dtype_str = dtype_str_obj.to_string_lossy();
    let dtype_char = dtype_str.chars().next().ok_or_else(|| {
        PyTypeError::new_err("Could not extract dtype character from numpy.ndarray")
    })?;

    // Map numpy dtype characters to TypeCode
    // numpy uses: 'b'=int8, 'h'=int16, 'i'=int32, 'l'=int64
    //             'B'=uint8, 'H'=uint16, 'I'=uint32, 'L'=uint64
    //             'f'=float32, 'd'=float64
    match dtype_char {
        'b' => Ok(TypeCode::Int8),
        'h' => Ok(TypeCode::Int16),
        'i' => Ok(TypeCode::Int32),
        'l' => Ok(TypeCode::Int64),
        'B' => Ok(TypeCode::UInt8),
        'H' => Ok(TypeCode::UInt16),
        'I' => Ok(TypeCode::UInt32),
        'L' => Ok(TypeCode::UInt64),
        'f' => Ok(TypeCode::Float32),
        'd' => Ok(TypeCode::Float64),
        _ => Err(PyTypeError::new_err(format!(
            "Unsupported numpy dtype: '{dtype_char}'. Supported: b, B, h, H, i, I, l, L, f, d"
        ))),
    }
}

/// Get typecode from memoryview format string
pub(crate) fn get_memoryview_typecode(mv: &Bound<'_, PyAny>) -> PyResult<TypeCode> {
    let format_attr = mv.getattr("format")?;
    let format_str_obj = format_attr.str()?;
    let format_str = format_str_obj.to_string_lossy();

    // Parse format string (handle endianness: '<i4', '>f8', 'i', 'I', etc.)
    // Remove endianness prefix if present (<, >, =, !)
    let cleaned_format = format_str.trim_start_matches(['<', '>', '=', '!']);

    // Extract the base type character (first character after endianness)
    let base_char = cleaned_format
        .chars()
        .next()
        .ok_or_else(|| PyTypeError::new_err("Could not parse memoryview format string"))?;

    // Map memoryview format to TypeCode
    // Standard formats: 'b'=signed char, 'B'=unsigned char, 'h'=short, 'H'=unsigned short
    //                   'i'=int, 'I'=unsigned int, 'l'=long, 'L'=unsigned long
    //                   'f'=float, 'd'=double
    match base_char {
        'b' => Ok(TypeCode::Int8),
        'B' => Ok(TypeCode::UInt8),
        'h' => Ok(TypeCode::Int16),
        'H' => Ok(TypeCode::UInt16),
        'i' => Ok(TypeCode::Int32),
        'I' => Ok(TypeCode::UInt32),
        'l' => Ok(TypeCode::Int64),
        'L' => Ok(TypeCode::UInt64),
        'f' => Ok(TypeCode::Float32),
        'd' => Ok(TypeCode::Float64),
        _ => Err(PyTypeError::new_err(format!(
            "Unsupported memoryview format: '{base_char}'. Supported: b, B, h, H, i, I, l, L, f, d"
        ))),
    }
}

/// Get typecode from Arrow buffer/array
pub(crate) fn get_arrow_typecode(arrow_obj: &Bound<'_, PyAny>) -> PyResult<TypeCode> {
    // Arrow arrays have a 'type' attribute with type information
    // For buffers, we need to check the underlying data type
    // Try to get dtype from Arrow array
    if let Ok(arrow_type) = arrow_obj.getattr("type") {
        if let Ok(type_str) = arrow_type.str() {
            let type_str = type_str.to_string_lossy();
            // Parse Arrow type string (e.g., "Int8", "Int32", "Float64")
            // Map to TypeCode
            if type_str.contains("Int8") || type_str.contains("int8") {
                return Ok(TypeCode::Int8);
            } else if type_str.contains("UInt8") || type_str.contains("uint8") {
                return Ok(TypeCode::UInt8);
            } else if type_str.contains("Int16") || type_str.contains("int16") {
                return Ok(TypeCode::Int16);
            } else if type_str.contains("UInt16") || type_str.contains("uint16") {
                return Ok(TypeCode::UInt16);
            } else if type_str.contains("Int32") || type_str.contains("int32") {
                return Ok(TypeCode::Int32);
            } else if type_str.contains("UInt32") || type_str.contains("uint32") {
                return Ok(TypeCode::UInt32);
            } else if type_str.contains("Int64") || type_str.contains("int64") {
                return Ok(TypeCode::Int64);
            } else if type_str.contains("UInt64") || type_str.contains("uint64") {
                return Ok(TypeCode::UInt64);
            } else if type_str.contains("Float32") || type_str.contains("float32") {
                return Ok(TypeCode::Float32);
            } else if type_str.contains("Float64")
                || type_str.contains("float64")
                || type_str.contains("Double")
            {
                return Ok(TypeCode::Float64);
            }
        }

        // Try to get id (Arrow type ID) as fallback
        if let Ok(id) = arrow_type.getattr("id") {
            if let Ok(id_val) = id.extract::<i32>() {
                // Arrow type IDs: Int8=2, Int16=3, Int32=4, Int64=5, UInt8=6, UInt16=7, UInt32=8, UInt64=9, Float32=10, Float64=11
                match id_val {
                    2 => return Ok(TypeCode::Int8),
                    3 => return Ok(TypeCode::Int16),
                    4 => return Ok(TypeCode::Int32),
                    5 => return Ok(TypeCode::Int64),
                    6 => return Ok(TypeCode::UInt8),
                    7 => return Ok(TypeCode::UInt16),
                    8 => return Ok(TypeCode::UInt32),
                    9 => return Ok(TypeCode::UInt64),
                    10 => return Ok(TypeCode::Float32),
                    11 => return Ok(TypeCode::Float64),
                    _ => {}
                }
            }
        }
    }

    // Note: Arrow buffers support buffer protocol, but type detection via buffer protocol
    // is complex. The type should be available via the Arrow type system above.

    Err(PyTypeError::new_err(
        "Could not determine Arrow buffer/array type. Supported types: Int8, UInt8, Int16, UInt16, Int32, UInt32, Int64, UInt64, Float32, Float64"
    ))
}
