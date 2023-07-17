use rand::{thread_rng, Rng};

pub struct DummyNetwork {
    outputs: usize,
}

impl DummyNetwork {
    pub fn new(outputs_num: usize) -> Self {
        Self {
            outputs: outputs_num,
        }
    }

    pub fn analize(&self) -> Vec<f32> {
        let mut outputs: Vec<f32> = vec![];
        let mut rng = thread_rng();
        for _ in 0..self.outputs {
            let out = rng.gen_range(-1.0..1.0);
            outputs.push(out);
        }
        return outputs;
    }
}
