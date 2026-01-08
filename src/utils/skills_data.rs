// src/pages/skills_data.rs

pub type SkillItem = (&'static str, &'static str);

#[derive(Clone, Copy)]
pub struct SkillGroup {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub items: &'static [SkillItem],
}

pub const LANGUAGES: &[SkillItem] = &[
    ("Python", "Strong"),
    ("Rust", "Solid"),
    ("C", "Solid"),
    ("Java", "Solid"),
    ("SQL", "Comfortable"),
    ("Bash", "Comfortable"),
    ("C#", "Learning"),
];

pub const CLOUD_ARCH_INFRA: &[SkillItem] = &[
    ("Cloud Architecture", "Solutions"),
    ("Landing Zones (accounts, network, guardrails)", "Design"),
    ("Compute / Storage / Networking", "Core"),
    ("VPC concepts (subnets, routing, NAT)", "Core"),
    ("IAM / Least Privilege", "Core"),
    ("HA / DR / Scalability", "Design"),
    ("Load Balancing", "Comfortable"),
    ("Observability (metrics/logs/tracing)", "Comfortable"),
    ("IaC (Terraform)", "Comfortable"),
];

pub const CLOUD_DATA_PLATFORMS: &[SkillItem] = &[
    ("Data Lake concepts", "Fundamentals"),
    ("Data Warehouse concepts", "Fundamentals"),
    ("Object Storage patterns (lifecycle, tiers)", "Comfortable"),
    ("ETL / ELT basics", "Familiar"),
    ("Data governance basics", "Familiar"),
];

pub const CONTAINERS_K8S: &[SkillItem] = &[
    ("Kubernetes", "Workflows"),
    ("Deployments / StatefulSets / Jobs", "Core"),
    ("Services / Ingress", "Core"),
    ("ConfigMaps / Secrets", "Core"),
    ("Helm", "Comfortable"),
    ("Argo CD (GitOps)", "Familiar"),
    ("Prometheus + Grafana", "Fundamentals"),
    ("cert-manager", "Familiar"),
    ("Istio", "Fundamentals"),
    ("Docker", "Daily"),
    ("Podman", "Comfortable"),
];

pub const LINUX_AUTOMATION: &[SkillItem] = &[
    ("Linux (Ubuntu/RHEL/Arch)", "Strong"),
    ("systemd", "Comfortable"),
    ("cron", "Comfortable"),
    ("CLI tooling (grep/sed/awk)", "Daily"),
    ("SSH", "Daily"),
    ("Networking tools (ip, ss, dig)", "Comfortable"),
    ("tcpdump / Wireshark", "Familiar"),
    ("Shell scripting", "Automation"),
    ("Git", "Daily"),
    ("CI basics (pipelines)", "Familiar"),
];

pub const DATA_MESSAGING: &[SkillItem] = &[
    ("PostgreSQL", "Comfortable"),
    ("MySQL", "Comfortable"),
    ("MongoDB", "Comfortable"),
    ("Redis", "Fundamentals"),
    ("Neo4j", "Fundamentals"),
    ("Kafka", "Fundamentals"),
    ("RabbitMQ", "Fundamentals"),
    ("OpenSearch / Elasticsearch", "Fundamentals"),
];

pub const AI_ML_TOOLING: &[SkillItem] = &[
    ("RAG pipelines", "Built"),
    ("Embeddings / reranking", "Comfortable"),
    ("Vector search (FAISS)", "Familiar"),
    ("OCR (Tesseract)", "Comfortable"),
    ("PyTorch", "Familiar"),
    ("Transformers (Hugging Face)", "Comfortable"),
    ("Whisper / ASR", "Familiar"),
    ("Evaluation mindset", "Practical"),
    ("Local-first tooling", "Enjoys"),
];

pub const GROUPS: &[SkillGroup] = &[
    SkillGroup {
        title: "Languages",
        subtitle: "Daily drivers and solid foundations",
        items: LANGUAGES,
    },
    SkillGroup {
        title: "Cloud and Infrastructure",
        subtitle: "Architecture, reliability, and core building blocks",
        items: CLOUD_ARCH_INFRA,
    },
    SkillGroup {
        title: "Cloud Data Platforms",
        subtitle: "Warehouses, lakes, and data workflows",
        items: CLOUD_DATA_PLATFORMS,
    },
    SkillGroup {
        title: "Kubernetes and Containers",
        subtitle: "Deploying, operating, and ecosystem fundamentals",
        items: CONTAINERS_K8S,
    },
    SkillGroup {
        title: "Linux and Automation",
        subtitle: "Systems, scripting, and practical operations",
        items: LINUX_AUTOMATION,
    },
    SkillGroup {
        title: "Data and Messaging",
        subtitle: "Databases and async architecture basics",
        items: DATA_MESSAGING,
    },
    SkillGroup {
        title: "AI and ML Tooling",
        subtitle: "Practical ML, OCR, and RAG-style systems",
        items: AI_ML_TOOLING,
    },
];
