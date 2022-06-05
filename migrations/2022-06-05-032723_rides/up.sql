CREATE TABLE rides (
    id serial NOT NULL,
    title character varying(255) NOT NULL,
    location character varying(255) NOT NULL,
    seconds INT NOT NULL,
    miles REAL NOT NULL,
    date DATE NOT NULL,
    elevation INT NOT NULL,
    average_speed REAL NOT NULL,
    average_power INT NOT NULL,
    CONSTRAINT workouts_pkey PRIMARY KEY (id)
)