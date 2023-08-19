CREATE USER app WITH PASSWORD 'Zn9cjJ!op4fz';

CREATE SCHEMA restaurant;

GRANT ALL PRIVILEGES ON SCHEMA restaurant TO app;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA restaurant TO app;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA restaurant TO app;

CREATE TABLE IF NOT EXISTS restaurant.items(
    item_id VARCHAR(100),
    name VARCHAR(200) NOT NULL,
    PRIMARY KEY(item_id),
    UNIQUE(name)
);

-- Insert items 
-- In production, this could be done using a control software or endpoints in the service.
INSERT INTO restaurant.items(item_id, name) VALUES('0', 'pizza');
INSERT INTO restaurant.items(item_id, name) VALUES('1', 'beer');
INSERT INTO restaurant.items(item_id, name) VALUES('2', 'burger');
INSERT INTO restaurant.items(item_id, name) VALUES('3', 'fries');
