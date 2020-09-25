FROM electronuserland/builder:wine

RUN apt-get update -y && \
  apt-get install -y --no-install-recommends mono-devel ca-certificates-mono tzdata && \
  apt-get clean && rm -rf /var/lib/apt/lists/*