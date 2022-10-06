// @generated automatically by Diesel CLI.
diesel::table! {
    product (PRODUCT_ID) {
        PRODUCT_ID -> Integer,
        PRODUCT_CODE -> Varchar,
        PRICE -> Double,
        NAME -> Varchar,
        LAST_UPDATE -> Date,
    }
}

diesel::table! {
    usuario (usuario_id) {
        usuario_id -> Integer,
        usuario_tipo_id -> Integer,
        usuario_nome -> Varchar,
        usuario_cpf -> Varchar,
        usuario_telefone -> Varchar,
    }
}

diesel::table! {
    usuario_tipo (usuario_tipo_id) {
        usuario_tipo_id -> Integer,
        usuario_tipo_desc -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    product,
    usuario,
    usuario_tipo,
);