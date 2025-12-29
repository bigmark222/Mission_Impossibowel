use burn::module::Module;
use burn::nn;
use burn::tensor::activation::relu;
use burn::tensor::Tensor;

#[derive(Debug, Clone)]
pub struct TinyDetConfig {
    pub hidden: usize,
}

impl Default for TinyDetConfig {
    fn default() -> Self {
        Self { hidden: 64 }
    }
}

#[derive(Debug, Module)]
pub struct TinyDet<B: burn::tensor::backend::Backend> {
    linear1: nn::Linear<B>,
    linear2: nn::Linear<B>,
}

impl<B: burn::tensor::backend::Backend> TinyDet<B> {
    pub fn new(cfg: TinyDetConfig, device: &B::Device) -> Self {
        let linear1 = nn::LinearConfig::new(4, cfg.hidden).init(device);
        let linear2 = nn::LinearConfig::new(cfg.hidden, 1).init(device);
        Self { linear1, linear2 }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(input);
        let x = relu(x);
        self.linear2.forward(x)
    }
}
