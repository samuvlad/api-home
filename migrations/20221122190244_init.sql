CREATE TABLE weather
(
    id  serial  PRIMARY KEY,
    created_at timestamp with time zone default CURRENT_TIMESTAMP,
    humidity FLOAT4,
    temperature FLOAT4
);

