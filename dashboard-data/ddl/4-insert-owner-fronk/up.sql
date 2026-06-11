/* insert java-hello app */
INSERT INTO applications (id,name) values (1,'java-hello');
INSERT INTO link (id,app_id,env_id,link_type,url) values (1000,1,900,101,'localhost:8101/actuator/info');
INSERT INTO link (id,app_id,env_id,link_type,url) values (1001,1,901,101,'localhost:8102/actuator/info');
INSERT INTO link (id,app_id,env_id,link_type,url) values (1002,1,902,101,'localhost:8103/actuator/info');

/* insert owner */
INSERT INTO owner (id,name) values (300,'fronk');
INSERT INTO ownership (id,app_id,owner_id) values (600,1,300);
