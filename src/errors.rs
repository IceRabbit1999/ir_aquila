pub trait StackError: std::error::Error {
    fn debug_fmt(&self, layer: usize, buf: &mut Vec<String>);

    fn next(&self) -> Option<&dyn StackError>;

    fn last(&self) -> &dyn StackError
    where
        Self: Sized,
    {
        let Some(mut result) = self.next() else {
            return self;
        };
        while let Some(err) = result.next() {
            result = err;
        }
        result
    }

    fn root_cause(&self) -> String
    where
        Self: Sized,
    {
        let last = self.last();
        let msg = format!("{}", last);

        // UNWRAP: Ok since we know the format of the error message string
        msg.split("error:").last().unwrap().trim().to_string()
    }
}