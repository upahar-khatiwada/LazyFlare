pub enum Qr {
    Query = 0,
    Response = 1,
}

pub enum Opcode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Notify = 4,
    Update = 5,
}

pub enum AuthoritativeAnswer {
    No = 0,
    Yes = 1,
}

pub enum Truncation {
    NotTruncated = 0,
    Truncated = 1,
}

pub enum RecursionDesired {
    No = 0,
    Yes = 1,
}

pub enum RecursionAvailable {
    No = 0,
    Yes = 1,
}

pub enum Z {
    Reserved = 0,
}

pub enum Rcode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}