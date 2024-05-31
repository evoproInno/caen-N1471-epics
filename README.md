# Introduction

This repository contains the EPICS IOC for the CAEN N1471 programmable HV power supply.

# Getting started

## 1. Setup EPICS environment

[Install EPICS, asyn and StreamDevice manually](https://docs.epics-controls.org/en/latest/getting-started/installation-linux.html) and change the installation paths in `configure/RELEASE`.

-- OR --

Use the provided devcontainer configuration.

## 2. Build the IOC

```
make
```

## 3. Testing

1. Start simulator (preferably with embedded socat)
```
cd caen-n1471-simulator && cargo run
```

2. Start IOC
```
cd iocBoot/caen-N1471-ioc && ./st.cmd
```

3. Install testing dependencies (as in [Dockerfile](.devcontainer/Dockerfile))
```
apt-get update --yes && apt-get install --yes --no-install-recommends \
    python3 \
    python-is-python3 \
    python3-pytest-bdd \
    python3-pyepics
```

4. Start pytest
```
pytest tests
```

# Useful links

- [Getting started with EPICS](https://docs.epics-controls.org/en/latest/getting-started/EPICS_Intro.html)
- [CAEN N1471 programmable HV power supply manual](https://eilabscom.sharepoint.com/:b:/r/sites/ENG/ISW/ISW_PRJ/2ddaqremotecontrol/Shared%20Documents/02-Knowledge_Base/04_HV_power_supply/N1471_rev19.pdf?csf=1&web=1&e=BAEGpe)
- [pytest-bdd](https://pytest-bdd.readthedocs.io/en/latest/#bdd-library-for-the-pytest-runner)