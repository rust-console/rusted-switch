FROM devkitpro/devkita64

#Install GCC for the CC link
RUN sudo apt-get update
RUN sudo apt-get install -y build-essential

#Install Rust
RUN curl https://sh.rustup.rs -sSf > rust-init.rs 
RUN chmod +x rust-init.rs 
RUN ./rust-init.rs -y 
RUN rm rust-init.rs
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly
RUN rustup component add rust-src
RUN cargo install xargo

#Mount the work directory
WORKDIR workdir 
VOLUME workdir 

CMD ./makew