--
-- PostgreSQL database dump
--

-- Dumped from database version 17.0
-- Dumped by pg_dump version 17.0

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    d_no integer NOT NULL,
    staff_sal real NOT NULL,
    age integer NOT NULL,
    mobile bigint NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, d_no, staff_sal, age, mobile) FROM stdin;
101	Alade Joy	2	250000	33	8023089664
100	Mustapha Ali	3	175000	32	8063285760
107	Alokwe Martin	7	380000	48	7090082816
97	Dankade Aminat	5	550000	40	9023688704
108	Josiah Joshua	1	120000	30	8053189120
102	Makinde Mary	2	450000	55	9023488000
120	Adeleke Jane	4	200000	38	7061045760
122	Osahon Mark	6	320000	44	8022289920
117	Suleman Ajayi	3	800000	50	7030089728
104	Kuti Lawal	1	750000	35	9145690112
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

