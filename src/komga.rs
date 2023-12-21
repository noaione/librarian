#[derive(Debug)]
pub struct KomgaClient {
    url: String,
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KomgaUser {
    pub id: String,
    pub email: String,
    pub roles: Vec<String>,
    #[serde(rename = "sharedAllLibraries")]
    pub shared_all_libraries: bool,
    #[serde(rename = "sharedLibrariesIds")]
    pub shared_libraries_ids: Vec<String>,
    #[serde(rename = "labelsAllow")]
    pub labels_allow: Vec<String>,
    #[serde(rename = "labelsExclude")]
    pub labels_exclude: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KomgaUserCreate {
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct KomgaUserCreateOptionSharedLibraries {
    pub all: bool,
    #[serde(rename = "libraryIds")]
    pub library_ids: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KomgaUserCreateOption {
    #[serde(rename = "labelsAllow")]
    pub labels_allow: Option<Vec<String>>,
    #[serde(rename = "labelsExclude")]
    pub labels_exclude: Option<Vec<String>>,
    #[serde(rename = "sharedLibraries")]
    pub shared_libraries: Option<KomgaUserCreateOptionSharedLibraries>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KomgaMinimalLibrary {
    pub id: String,
    pub name: String,
    pub unavailable: bool,
}

impl KomgaClient {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
        }
    }

    pub fn instance() -> Self {
        let komga_host = std::env::var("KOMGA_HOST").expect("KOMGA_HOST not set");
        let komga_username = std::env::var("KOMGA_USERNAME").expect("KOMGA_USERNAME not set");
        let komga_password = std::env::var("KOMGA_PASSWORD").expect("KOMGA_PASSWORD not set");

        Self::new(
            komga_host.clone(),
            komga_username.clone(),
            komga_password.clone(),
        )
    }

    pub async fn get_me(&self) -> anyhow::Result<KomgaUser> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{}/api/v2/users/me", self.url))
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        let user: KomgaUser = res.json().await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: KomgaUserCreate) -> anyhow::Result<KomgaUser> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/api/v2/users", self.url))
            .basic_auth(&self.username, Some(&self.password))
            .json(&user)
            .send()
            .await?;

        let user: KomgaUser = res.json().await?;

        Ok(user)
    }

    pub async fn apply_user_restriction(
        &self,
        user_id: String,
        option: KomgaUserCreateOption,
    ) -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        let res = client
            .patch(format!("{}/api/v2/users/{}", self.url, user_id))
            .basic_auth(&self.username, Some(&self.password))
            .json(&option)
            .send()
            .await?;

        let status_code = res.status();

        if status_code.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to apply user restriction"))
        }
    }

    pub async fn get_sharing_labels(&self) -> anyhow::Result<Vec<String>> {
        let client = reqwest::Client::new();

        let res = client
            .get(format!("{}/api/v1/sharing-labels", self.url))
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        let labels: Vec<String> = res.json().await?;

        Ok(labels)
    }

    pub async fn get_libraries(&self) -> anyhow::Result<Vec<KomgaMinimalLibrary>> {
        let client = reqwest::Client::new();

        let res = client
            .get(format!("{}/api/v1/libraries", self.url))
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        let libraries: Vec<KomgaMinimalLibrary> = res.json().await?;

        Ok(libraries)
    }

    pub fn get_host(&self) -> String {
        self.url.clone()
    }
}
