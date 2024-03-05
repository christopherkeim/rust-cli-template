#! /bin/bash
# 
# This script sets up a local development environment on an Ubuntu 20.04/22.04 machine
# to work with Rust (cargo 1.76.0) projects. 
# 
# Targets:
#   - cargo 1.76.0
#   - Docker 24.0.6 (optional with `--docker`)
#
# Requirements:
#   - Ubuntu 20.04/22.04
#

# -----------------------------------------------------------------------------------------------------------
# 0) Config: here we'll set up default variables.
# -----------------------------------------------------------------------------------------------------------

# Pull the current machine's distro for GPG key targeting
readonly DISTRO="$(lsb_release -d | awk -F ' ' '{print tolower($2)}')"

readonly PACKAGE="setup.sh"

WITH_DOCKER=false

show_help() {
  cat <<END
  Sets up Rust (cargo 1.76.0) environment for command-line 
  application development on Ubuntu 20.04/22.04.

  ${PACKAGE} [options] [arguments]

  options:
  -h, --help                Display help message
  -d, --docker              Includes Docker 24.0.6 in installation
END

  exit 0
}

while getopts 'hd-:' flag; do
  case "${flag}" in
    h) show_help ;;
    d) WITH_DOCKER=true ;;
    -)
      # Prefix substring removal matching "*="
      LONG_OPTARG_VALUE="${OPTARG#*=}"
      case ${OPTARG} in
        help)
          show_help
          ;;
        docker)
          WITH_DOCKER=true
          ;;
        docker*)
          echo "No arg for --${OPTARG} option" >&2
          exit 2
          ;;
        help*)
          echo "No arg allowed for --${OPTARG} option" >&2; 
          exit 2
          ;;
        *)
          echo "Unknown option --$OPTARG" >&2
          exit 2 
          ;;
      esac 
      ;;
    # getopts already reported the unknown option error, so exit
    \?) exit 2 ;;
  esac
done
shift $((OPTIND-1))

readonly WITH_DOCKER


# -----------------------------------------------------------------------------------------------------------
# 1) Base Requirements: this will ensure that you have some base requirements installed.
# -----------------------------------------------------------------------------------------------------------

# Check if ca-certificates is in the apt-cache
if ( apt-cache show ca-certificates > /dev/null ); then
  echo 'ca-certificates is already cached 🟢'
else
  sudo apt update
fi

# Ensure ca-certificates package is installed on the machine
if ( which update-ca-certificates > /dev/null ); then
  echo 'ca-certificates is already installed 🟢'
else
  echo 'Installing ca-certificates 📜'
  sudo apt-get install -y ca-certificates
fi

# Check if curl is in the apt-cache
if ( apt-cache show curl > /dev/null ); then
  echo 'curl is already cached 🟢'
else
  sudo apt update
fi

# Ensure curl is installed on the machine
if ( which curl > /dev/null ); then
  echo 'curl is already installed 🟢'
else
  echo 'Installing curl 🌀'
  sudo apt install -y curl
fi

# Check if make is in the apt-cache
if ( apt-cache show make > /dev/null ); then
  echo 'make is already cached 🟢'
else
  sudo apt update
fi

# Ensure make is installed on the machine
if ( which make > /dev/null ); then
  echo 'make is already installed 🟢'
else
  echo 'Installing make 🔧'
  sudo apt install -y make
fi

# Check if gnupg is in the apt-cache
if ( apt-cache show gpg > /dev/null ); then
  echo 'gnupg is already cached 🟢'
else
  sudo apt update
fi

# Ensure gnupg is installed on the machine
if ( which gpg > /dev/null ); then
  echo 'gnupg is already installed 🟢'
else
  echo 'Installing gnugp 🔧'
  sudo apt install -y gnupg
fi


# -----------------------------------------------------------------------------------------------------------
# 2) Rust Install: here we'll install and configure Cargo.
# -----------------------------------------------------------------------------------------------------------

# Install Cargo
if ( which cargo > /dev/null ); then
  echo "Cargo is already installed 🟢"
else
  echo "Installing Rust 🦀"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y
  source ~/.bashrc
fi

# Verify installation of Cargo
if ( cargo version > /dev/null ); then
  echo "$(cargo version) 🦀 🚀"
else
  echo "Cargo was not installed successfully 🔴"
fi


# -----------------------------------------------------------------------------------------------------------
# 3) Docker Install: here we'll install Docker
# -----------------------------------------------------------------------------------------------------------

if [[ $WITH_DOCKER == false ]]; then
  exit 0
fi

# -----------------------------------------------------------------------------------------------------------
# 3.1) Set up the repository: Before you install Docker Engine for the first time on a new host machine, 
# you need to set up the Docker repository. Afterward, you can install and update Docker from the repository.
# -----------------------------------------------------------------------------------------------------------

# Add Docker’s official GPG key
if [[ -f /etc/apt/keyrings/docker.gpg ]]; then
  echo 'Docker GPG Key already installed at /etc/apt/keyrings/docker.gpg 🟢'
else
  echo 'Installing Docker GPG Key at /etc/apt/keyrings/docker.gpg 🔧'
  
  # Create the /etc/apt/keyrings directory with appropriate permissions
  sudo install -m 0755 -d /etc/apt/keyrings
  
  # Download the GPG key from Docker
  curl -fsSL https://download.docker.com/linux/${DISTRO}/gpg \
    | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

  sudo chmod a+r /etc/apt/keyrings/docker.gpg
fi

# Set up the repository
if [[ -f /etc/apt/sources.list.d/docker.list ]]; then
  echo 'docker.list repository already exists at /etc/apt/sources.list.d/docker.list 🟢'
else
  echo 'Installing docker.list repository at /etc/apt/sources.list.d/docker.list 🔧'
  echo \
    "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] \
    https://download.docker.com/linux/$DISTRO \
    "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" \
    | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
fi

# -----------------------------------------------------------------------------------------------------------
# 3.2) Install Docker Engine
# -----------------------------------------------------------------------------------------------------------

# Check if docker-ce is in the apt-cache
if ( apt-cache show docker-ce > /dev/null ); then
  echo 'docker-ce is already cached 🟢'
else
  sudo apt update
fi

# Install Docker Engine, containerd, and Docker Compose
if ( docker --version > /dev/null ); then
  echo 'Docker is already installed 🟢'
  echo "Using $(docker --version)"
else
  echo 'Installing Docker 🐳'

  # Installs
  sudo apt-get install -y docker-ce \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin
  
  # Verify that the Docker Engine installation is successful by running the hello-world image
  sudo docker run --rm hello-world
fi