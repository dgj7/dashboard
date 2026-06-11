INSERT INTO environments (id,name) values (900,'dev');
INSERT INTO environments (id,name) values (901,'uat');
INSERT INTO environments (id,name) values (902,'prd');

INSERT INTO link_type (id,name) values (100,'other');
INSERT INTO link_type (id,name) values (101,'liveness');    -- app is running
INSERT INTO link_type (id,name) values (102,'readiness');   -- app is ready for traffic
INSERT INTO link_type (id,name) values (103,'splunk');      -- where logs live
INSERT INTO link_type (id,name) values (104,'git');         -- where code lives
