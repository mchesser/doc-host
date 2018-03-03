use std::env;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub tmp_dir: PathBuf,
    pub author_name: String,
    pub author_email: String,
    pub credentials: GitCredentials,
    pub src: GitSource,
    pub dst: GitSource,
}

#[derive(Clone)]
pub struct GitSource {
    pub url: String,
    pub branch: String,
}

#[derive(Clone)]
pub struct GitCredentials {
    pub username: String,
    pub ssh_key_path: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            tmp_dir: PathBuf::from(env::var("TMP_DIR").unwrap_or_else(|_| "tmp".into())),
            author_name: env::var("GIT_AUTHOR").expect("`GIT_AUTHOR` not set"),
            author_email: env::var("GIT_EMAIL").expect("`GIT_EMAIL` not set"),
            credentials: GitCredentials {
                username: "git".into(),
                ssh_key_path: env::var("SSH_KEY_PATH").expect("SSH_KEY_PATH`` not set"),
            },
            src: GitSource {
                url: env::var("SRC_URL").expect("`SRC_URL` not set"),
                branch: env::var("SRC_BRANCH").unwrap_or_else(|_| "master".into()),
            },
            dst: GitSource {
                url: env::var("DST_URL").expect("`DST_URL` not set"),
                branch: env::var("DST_BRANCH").unwrap_or_else(|_| "gh-pages".into()),
            }
        }
    }
}
