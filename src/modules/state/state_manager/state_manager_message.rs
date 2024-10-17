pub enum StateManagerMessage<T> {
    GET {
        state_ident: String,
    },
    POST{
        state_ident: String,
        value: T
    },
    PUT{
        state_ident: String,
        value: T
    },
    DELETE{
        state_ident: String,
    },
}