CREATE TABLE app.maintenance_logs (
  id               SERIAL PRIMARY KEY,
  motorcycle_id    INTEGER       NOT NULL
                      REFERENCES app.motorcycles(id)
                      ON DELETE CASCADE,
  activity_date    DATE          NOT NULL,
  activity_type    VARCHAR(100)  NOT NULL,   -- e.g. “oil change”, “chain adjustment”
  mileage_km       INTEGER,                    -- odometer reading
  description      TEXT,                       -- free-text notes
  logged_at        TIMESTAMPTZ   NOT NULL DEFAULT now()
);

ALTER TABLE app.motorcycles
ADD created_at     TIMESTAMPTZ   NOT NULL DEFAULT now();

GRANT ALL ON app.maintenance_logs TO app_test;
GRANT ALL ON app.maintenance_logs TO admin_test;
