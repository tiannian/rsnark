use rsnark_core::{API, Circuit, CircuitDefine};

// 使用宏自动生成 CircuitElement 实现
#[derive(Circuit)]
pub struct MacroCircuit {
    a: u32,
    pub d: u32,
    b: u32,
    pub c: u32,
}

// 只需要实现 Circuit trait，其他的都由宏自动生成
impl Circuit for CircuitDefine<MacroCircuit> {
    fn define(&self, api: &mut impl API) {
        let c = api.add(&self.a, &self.b, &[]);
        api.assert_is_equal(&c, &self.c);
    }
}

fn main() {
    println!("Macro-generated circuit works!");
}
