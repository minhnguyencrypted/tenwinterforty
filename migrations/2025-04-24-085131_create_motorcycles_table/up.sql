CREATE TABLE app.motorcycles (
    id SERIAL PRIMARY KEY,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    year INT CHECK (year >= 1900),
    license_plate TEXT UNIQUE NOT NULL,
    displacement INT CHECK (displacement > 0)
);

GRANT ALL ON app.motorcycles TO app_test;
GRANT ALL ON app.motorcycles TO admin_test;

