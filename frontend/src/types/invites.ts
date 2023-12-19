export interface InviteSharedLibrary {
  all: boolean;
  libraryIds: string[];
}

export interface InviteOption {
  labelsAllow: string[] | null;
  labelsExclude: string[] | null;
  sharedLibraries: InviteSharedLibrary | null;
  expiresAt: number | null;
  roles: string[] | null;
}

export interface Invite {
  token: string;
  option: InviteOption;
  user_id: string | null;
}

export interface InviteConfig {
  libraries: {
    id: string;
    name: string;
    unavailable: boolean;
  }[];
  labels: string[];
}
