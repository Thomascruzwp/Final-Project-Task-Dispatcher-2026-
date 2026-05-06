pub struct Experiment{
    pub name: & 'static str,
    pub tasks: usize,
}

pub fn exp_a() -> Experiment{
    Experiment{
        name: "FIFO simulation", tasks: 500
    }
}

pub fn exp_b() -> Experiment{
    Experiment{
        name: "Optimized simulation", tasks: 1000,
    }
}