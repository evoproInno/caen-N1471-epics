FROM debian:bookworm

ARG EPICS_INSTALL_DIR="/EPICS"
ARG EPICS_BASE_VERSION="R7.0.8"
ARG EPICS_ASYN_VERSION="R4-44-2"
ARG EPICS_STREAMDEVICE_VERSION="2.8.24"
ARG PHOEBUS_VERSION="v4.7.3"
ARG PVXS_VERSION="1.3.1"

ENV DEBIAN_FRONTEND="noninteractive"

# Requirements for building EPICS
RUN apt-get update --yes && apt-get install --yes --no-install-recommends \
    build-essential \
    git \
    ca-certificates

WORKDIR ${EPICS_INSTALL_DIR}

# Clone epics-base
RUN git clone \
    --branch ${EPICS_BASE_VERSION} \
    --single-branch \
    --depth 1 \
    --recursive \
    https://github.com/epics-base/epics-base.git \
    && cd epics-base && make

# Add EPICS environment variables
RUN echo "export EPICS_BASE=${EPICS_INSTALL_DIR}/epics-base" > ${HOME}/.bashrc \
    && echo 'export EPICS_HOST_ARCH=$(${EPICS_BASE}/startup/EpicsHostArch)' >> ${HOME}/.bashrc \
    && echo 'export PATH=${EPICS_BASE}/bin/${EPICS_HOST_ARCH}:${PATH}' >> ${HOME}/.bashrc

# Clone asyn
RUN git clone \
    --branch ${EPICS_ASYN_VERSION} \
    --single-branch \
    --depth 1 \
    https://github.com/epics-modules/asyn.git

# Setup asyn configuration: epics-base location and enable linking to libtirpc
RUN sed -i "s|^#EPICS_BASE=.*|EPICS_BASE=${EPICS_INSTALL_DIR}/epics-base|" ./asyn/configure/RELEASE \
    && sed -i 's|^# TIRPC=YES|TIRPC=YES|' ./asyn/configure/CONFIG_SITE

# Build asyn
RUN cd ./asyn && make

# Clone StreamDevice
RUN git clone \
    --branch ${EPICS_STREAMDEVICE_VERSION} \
    --single-branch \
    --depth 1 \
    https://github.com/paulscherrerinstitute/StreamDevice.git

# Setup StreamDevice configuration: epics-base and asyn location, disable CALC and PCRE
RUN sed -i "s|^EPICS_BASE=.*|EPICS_BASE=${EPICS_INSTALL_DIR}/epics-base|" ./StreamDevice/configure/RELEASE \
    && sed -i "s|^ASYN=.*|ASYN=${EPICS_INSTALL_DIR}/asyn|" ./StreamDevice/configure/RELEASE \
    && sed -i 's|^CALC=.*|#&|' ./StreamDevice/configure/RELEASE \
    && sed -i 's|^PCRE=.*|#&|' ./StreamDevice/configure/RELEASE

# Build StreamDevice
RUN cd ./StreamDevice && make

# Requirements for installing rust
RUN apt-get update --yes && apt-get install --yes --no-install-recommends \
    curl

# Install rust with clippy and rustfmt
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --quiet -y --component rustfmt,clippy

# Requirements for PVXS for PV access protocol
RUN apt-get update --yes && apt-get install --yes --no-install-recommends \
    libevent-dev

# Clone PVXS
RUN git clone \
    --branch ${PVXS_VERSION} \
    --single-branch \
    --depth 1 \
    --recursive \
    https://github.com/mdavidsaver/pvxs.git

# Setup PVXS configuration: epics-base location
RUN sed -i "s|^#EPICS_BASE.*|EPICS_BASE=${EPICS_INSTALL_DIR}/epics-base|" ./pvxs/configure/RELEASE

# Build PVXS
RUN cd ./pvxs && make

# Requirements for BDD testing using pytest-bdd
RUN apt-get update --yes && apt-get install --yes --no-install-recommends \
    socat \
    python3 \
    python-is-python3 \
    python3-pip

COPY requirements.txt /tmp/
RUN pip install -r /tmp/requirements.txt --break-system-packages

# Requirements for graphical operator interface development
RUN apt-get update --yes && apt-get install --yes --no-install-recommends \
    openjdk-17-jdk \
    maven

# Clone phoebus
RUN git clone \
    --branch ${PHOEBUS_VERSION} \
    --single-branch \
    --depth 1 \
    https://github.com/ControlSystemStudio/phoebus

# Build phoebus
RUN cd ./phoebus && mvn clean verify -f dependencies/pom.xml && JAVA_HOME=/lib/jvm/java-17-openjdk-amd64 mvn -DskipTests clean install

# Extra utilities - add your favourite packages here
RUN apt-get update --yes && apt-get install --yes --no-install-recommends \
    vim \
    openssh-client \
    locales-all

COPY .devcontainer/start.sh /tmp/
RUN chmod +x /tmp/start.sh
ENTRYPOINT ["/tmp/start.sh"]

CMD ["bash"]
