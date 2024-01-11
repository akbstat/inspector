# Inspector

products
* acrf
* spec
* code(domain.sas, title.sas)
* dataset(domain.sas7bdat, title.sas7bdat)
* output(title.rtf)

validations
* code(v-domain.sas, v-title.sas)
* dataset(v-domain.sas7bdat, v_title.sas7bdat)
* output(v-title.rtf)
* qc-result(v_title.rtf)

process:
- SDTM
    1. acrf (review) dir = "/documents/crf/xxxx acrf.pdf"
    2. spec (review)
    3. coding phase
    4. p21
    5. output review phase(review)
    6. complete
- ADAM
    1. spec (review)
    2. coding phase
    3. p21
    4. output review phase(review)
    5. complete
- TFL
    1. top (review)
    2. coding phase
    4. output review phase(review)
    5. complete