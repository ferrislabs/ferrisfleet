use snafu::Snafu;

#[derive(Debug, Snafu)]

pub enum FleetError {
    #[snafu(display("Internal server error {}", message))]
    InternalServerError { message: String },
}
