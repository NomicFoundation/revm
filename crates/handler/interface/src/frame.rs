use crate::FrameOrResultGen;

/// Call frame trait
pub trait Frame: Sized {
    type Context<'context>;
    type FrameInit;
    type FrameResult;
    type Error;

    fn init_first(
        context: &mut Self::Context<'_>,
        frame_input: Self::FrameInit,
    ) -> Result<FrameOrResultGen<Self, Self::FrameResult>, Self::Error>;

    fn final_return(
        context: &mut Self::Context<'_>,
        result: &mut Self::FrameResult,
    ) -> Result<(), Self::Error>;

    fn init(
        &self,
        context: &mut Self::Context<'_>,
        frame_input: Self::FrameInit,
    ) -> Result<FrameOrResultGen<Self, Self::FrameResult>, Self::Error>;

    fn run(
        &mut self,
        context: &mut Self::Context<'_>,
    ) -> Result<FrameOrResultGen<Self::FrameInit, Self::FrameResult>, Self::Error>;

    fn return_result(
        &mut self,
        context: &mut Self::Context<'_>,
        result: Self::FrameResult,
    ) -> Result<(), Self::Error>;
}
