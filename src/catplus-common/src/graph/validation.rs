/// Interface for validating an RDF graph.


pub trait GraphValidator {
    fn validate(&self, data: &Graph, shapes: &Graph) -> Result<(), GraphValidationError>;

}

pub struct GraphValidationError {
    pub message: String,
}

pub struct ShaclApiEndpoint {
    url: String,
}

impl ShaclApiEndpoint {
    pub fn new(url: String) -> Self {
        ShaclApiEndpoint { url }
    }
}

impl GraphValidator for ShaclApiEndpoint {
    fn validate(&self, data: &Graph, shapes: &Graph) -> Result<(), GraphValidationError> {
        // serialize graphs to ttl
        // base64 encode ttl
        // send POST request /validate
        // body should be {datafile: <data>, shapesfile: <data>}
        Ok(())
    }
}
