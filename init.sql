-- -- USE THIS if you are manually initiating DB instead of using Docker
-- SHOW server_encoding;
-- SHOW client_encoding;
-- SET client_encoding TO 'UTF8';

-- -- Create user
-- CREATE ROLE YOUR_NAME WITH PASSWORD 'YOUR_PASS' LOGIN;

-- -- Create DB
-- CREATE DATABASE YOUR_DATABASE WITH OWNER YOUR_NAME
-- TEMPLATE 'template0'
-- ENCODING 'UTF8'
-- LC_COLLATE 'C'
-- LC_CTYPE 'en_US.UTF-8';

-- \c YOUR_DATABASE
-- SET ROLE YOUR_NAME;

CREATE TABLE company (
  id SERIAL PRIMARY KEY,
  srtn_cd CHAR(6) UNIQUE,
  isin_cd CHAR(12),
  mrkt_ctg VARCHAR(6),
  itms_nm VARCHAR(240),
  crno VARCHAR(20),
  corp_nm VARCHAR(240)
);

CREATE TABLE dart_code (
  id SERIAL PRIMARY KEY,
  code CHAR(8),
  name TEXT,
  date DATE
);

CREATE TABLE price (
  id SERIAL PRIMARY KEY,
  bas_dt DATE,
  srtn_cd CHAR(6),
  isin_cd CHAR(12),
  itms_nm VARCHAR(120),
  mrkt_ctg VARCHAR(6),
  clpr INTEGER,
  vs INTEGER,
  flt_rt REAL,
  mkp INTEGER,
  hipr INTEGER,
  lopr INTEGER,
  trqu DECIMAL,
  tr_prc DECIMAL,
  lstg_st_cnt DECIMAL,
  mrkt_tot_amt DECIMAL,
  UNIQUE(bas_dt, srtn_cd)
);

CREATE TABLE price_weekly (
  id SERIAL PRIMARY KEY,
  srtn_cd CHAR(6),
  year INTEGER,
  week INTEGER,
  opening_date DATE,
  closing_date DATE,
  open DECIMAL,
  close DECIMAL,
  high DECIMAL,
  low DECIMAL,
  volume DECIMAL,
  trading_value DECIMAL,
  base_stock_cnt DECIMAL,
  UNIQUE(srtn_cd, year, week)
);


-------------------- US Stock --------------------
CREATE TABLE ticker (
  id SERIAL PRIMARY KEY,
  cik_str CHAR(10),
  ticker TEXT,
  title TEXT
);
