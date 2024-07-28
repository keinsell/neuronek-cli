all:
	sea migrate fresh && \
	sea generate entity -o entity -l --with-copy-enums --compact-format --with-serde both --date-time-crate chrono -v
