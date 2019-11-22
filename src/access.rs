// this module is allows the Database object to
// authenticate at the external database server
pub struct Access<A> {
    pub project_id: String,
    pub access: A,
}

unsafe impl <A: Send> std::marker::Send for Access<A> {}
