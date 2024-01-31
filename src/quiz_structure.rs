
#[derive(Debug)]
pub struct Quiz{
    pub expr:L1,
    pub answer:i16
}
#[derive(Debug)]
pub enum L1Ops{
    Add,
    Sub
}
#[derive(Debug)]
struct L1{
    pub op:L1Ops,
    pub   left:L1Arm,
    pub  right:L1Arm
}
#[derive(Debug)]
pub enum L1Arm{
    Num(i16),
    L1(Box<L1>),
    L2(L2)
}
#[derive(Debug)]
pub enum L2Ops{
    Div
    ,Mul
}
#[derive(Debug)]
pub enum L2Arm{
    Num(i16),
    L2(Box<L2>)
}

#[derive(Debug)]
pub struct L2{
    pub  op:L2Ops,
    pub   left:L2Arm,
    pub  right:L2Arm
}

