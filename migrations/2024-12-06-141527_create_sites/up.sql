CREATE TABLE sites (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    purpose TEXT NOT NULL,
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    start_day TIMESTAMP NOT NULL,
    duration_half_day INTEGER NOT NULL,
    start_period ENUM('Morning', 'Afternoon') NOT NULL,
    status ENUM('NotCarried', 'InProgress', 'Interrupted', 'Completed') NOT NULL,
    site_manager_id BIGINT UNSIGNED NOT NULL,
    client_id BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (site_manager_id) REFERENCES site_managers(id),
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

-- Resources
CREATE TABLE site_vehicles (
    site_id BIGINT UNSIGNED NOT NULL,
    vehicle_id BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE,
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id) ON DELETE CASCADE,
    PRIMARY KEY (site_id, vehicle_id)
);

CREATE TABLE site_workers (
    site_id BIGINT UNSIGNED NOT NULL,
    worker_id BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (site_id) REFERENCES sites(id) ON DELETE CASCADE,
    FOREIGN KEY (worker_id) REFERENCES workers(id) ON DELETE CASCADE,
    PRIMARY KEY (site_id, worker_id)
);