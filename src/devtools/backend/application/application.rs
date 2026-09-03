use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ServiceWorker {
    pub registration_id: String,
    pub scope: String,
    pub script_url: String,
    pub state: String,
    pub version_id: String,
}

#[derive(Debug, Clone)]
pub struct Manifest {
    pub start_url: String,
    pub display: String,
    pub orientation: String,
    pub background_color: Option<String>,
    pub theme_color: Option<String>,
}

pub struct ApplicationBackend {
    pub service_workers: Vec<ServiceWorker>,
    pub manifest: Option<Manifest>,
}

impl ApplicationBackend {
    pub fn new() -> Self {
        ApplicationBackend {
            service_workers: Vec::new(),
            manifest: None,
        }
    }

    pub fn register_service_worker(&mut self, registration_id: &str, scope: &str, script_url: &str) {
        self.service_workers.push(ServiceWorker {
            registration_id: registration_id.to_string(),
            scope: scope.to_string(),
            script_url: script_url.to_string(),
            state: "installed".to_string(),
            version_id: format!("v{}", self.service_workers.len() + 1),
        });
    }

    pub fn get_service_workers(&self) -> &[ServiceWorker] {
        &self.service_workers
    }

    pub fn unregister_service_worker(&mut self, registration_id: &str) -> bool {
        let len = self.service_workers.len();
        self.service_workers.retain(|sw| sw.registration_id != registration_id);
        len != self.service_workers.len()
    }

    pub fn set_manifest(&mut self, manifest: Manifest) {
        self.manifest = Some(manifest);
    }

    pub fn get_manifest(&self) -> Option<&Manifest> {
        self.manifest.as_ref()
    }

    pub fn to_json(&self) -> Value {
        let workers: Vec<Value> = self.service_workers.iter().map(|sw| {
            serde_json::json!({
                "registrationId": sw.registration_id,
                "scope": sw.scope,
                "scriptURL": sw.script_url,
                "state": sw.state,
                "versionId": sw.version_id
            })
        }).collect();
        let manifest_json = if let Some(m) = &self.manifest {
            serde_json::json!({
                "startURL": m.start_url,
                "display": m.display,
                "orientation": m.orientation,
                "backgroundColor": m.background_color,
                "themeColor": m.theme_color
            })
        } else {
            serde_json::Value::Null
        };
        serde_json::json!({
            "serviceWorkers": workers,
            "manifest": manifest_json
        })
    }
}

impl Default for ApplicationBackend {
    fn default() -> Self {
        Self::new()
    }
}
