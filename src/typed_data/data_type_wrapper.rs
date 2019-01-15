use types::DataType;

pub trait DataTypeWrapper<T> {
    fn data_type(&self) -> &DataType;
}

pub trait WrappableData {
    fn data_size(&self) -> Option<usize> {
        None
    }
}
