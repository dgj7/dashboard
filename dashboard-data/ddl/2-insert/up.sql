INSERT INTO applications (id,name) values (1,'java-hello');

INSERT INTO link_type (id,name) values (100,'other');
INSERT INTO link_type (id,name) values (101,'liveness');    -- app is running
INSERT INTO link_type (id,name) values (102,'readiness');   -- app is ready for traffic
INSERT INTO link_type (id,name) values (103,'splunk');      -- where logs live
INSERT INTO link_type (id,name) values (104,'git');         -- where code lives

INSERT INTO link (id,app_id,link_type,url) values (1000,1,101,'localhost:8101/actuator/info');

INSERT INTO owner (id,name) values (300,'fronk');

INSERT INTO ownership (id,app_id,owner_id) values (600,1,300);

