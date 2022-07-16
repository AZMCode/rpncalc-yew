#[derive(Default)]
pub struct CalcUnit {
    stack: Vec<f64>
}

impl CalcUnit {
    pub fn run_command(&mut self, comm: rpncalc::CommandOrOp) -> (Vec<u8>,rpncalc::error::Result<Option<String>>) {
        use rpncalc::Command;
        let mut stdout = std::io::Cursor::new(vec![]);
        let out = comm.comm(&mut self.stack, std::io::empty(),&mut stdout);
        (stdout.into_inner(),out)
    }
    pub fn get_stack(&self) -> &[f64] {
        &self.stack
    }
}