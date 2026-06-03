CREATE TABLE IF NOT EXISTS applications (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS link_type (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS link (
    id INTEGER PRIMARY KEY NOT NULL,
    app_id INTEGER NOT NULL,
    link_type INTEGER NOT NULL,
    url VARCHAR(100) NOT NULL,
    FOREIGN KEY (app_id) REFERENCES applications(id) ON DELETE CASCADE ON UPDATE CASCADE,
    FOREIGN KEY (link_type) REFERENCES link_type(id) ON DELETE CASCADE ON UPDATE CASCADE
);

INSERT INTO applications (id,name) values (1,'java-hello');

INSERT INTO link_type (id,name) values (100,'other');
INSERT INTO link_type (id,name) values (101,'liveness');    -- app is running
INSERT INTO link_type (id,name) values (102,'readiness');   -- app is ready for traffic
INSERT INTO link_type (id,name) values (103,'splunk');      -- where logs live
INSERT INTO link_type (id,name) values (104,'git');         -- where code lives

INSERT INTO link (id,app_id,link_type,url) values (1000,1,101,'localhost:8101/actuator/info');
