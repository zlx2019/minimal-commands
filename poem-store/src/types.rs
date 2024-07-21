use std::error::Error;

/// 自定义类型

// 错误类型包装
pub type DynamicError = Box<dyn Error>;
// Result包装
pub type DefaultResult = Result<(), DynamicError>;