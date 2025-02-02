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
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text,
    c_mobile bigint,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Mustafa Karim	35	m_karim@gmail.com	8055089152	102	5
111	Lilian Jaiya	43	i_jaiya@gmail.com	8055185408	100	3
112	Arthur Musa	50	a_musa@gmail.com	7055282688	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	9052356608	100	2
114	Maryiene Mapa	33	m_mapa@gmail.com	8053333504	120	5
115	Oghenero Agor	50	o_agor@gmail.com	7055566848	117	11
116	Adams Bree	33	a_bree@gmail.com	8056765440	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	8056763392	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	7056774656	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	9052110848	107	5
120	James Job	44	j_job@gmail.com	8059694080	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	7051232256	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	8054921728	107	5
\.


--
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (c_id);


--
-- Name: customer_table customer_table_eid_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_eid_fkey FOREIGN KEY (eid) REFERENCES public.staff(staff_id);


--
-- PostgreSQL database dump complete
--

