# Distributed LLM Inference and Data Processing Platform

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-Kubernetes-orange)

---

# overview
This project aims to utilize Hadoop and Spark for large-scale data processing, write high-performance middleware and apis with RustGo, conduct container orchestration and deployment with Kubernetes, and integrate LLM inference optimization techniques (such as model quantization). Build LLM-based applications using dynamic batch processing and LangChain, etc


---

# structure

```
.
├── config
├── docker-compose.yml # Local development stack (Spark, Redis, MinIO, etc.)
├── docs # Project documentation
│   ├── api # API reference or design docs
│   └── architecture # System architecture diagrams and technical specs
├── k8s # Kubernetes deployment configurations
│   └── manifests # YAML manifests for Deployments, Services, etc.
├── LICENSE
├── Makefile # Build and deployment automation commands
├── README.md # Project introduction and setup guide
├── requirements.txt # Python dependencies for the inference and LangChain modules
├── scripts # Utility scripts for setup, testing, or maintenance
└── src # Source code of all core services
    ├── api-gateway # Rust-based API Gateway (handles routing and orchestration)
    ├── inference-engine # FastAPI + PyTorch inference service
    ├── langchain-app # LangChain logic for agent reasoning and memory
    └── spark-jobs # Distributed data processing jobs for Spark
```

---

# how to run

```
# git clone
git clone https://github.com/hortus-neu/DistriLLM.git
cd DistriLLM

# run dev
make dev-setup

# deploy basical set-ups
make deploy-local
```

---

# documents


- [中文版本](docs/architecture/system-design.zh.md)
- [English Version](docs/architecture/system-design.en.md)



