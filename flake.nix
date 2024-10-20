{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
          workspaceSrc = ./.;
        };

      in rec {
        packages = {
          # replace hello-world with your package name
          fire_alarm = (rustPkgs.workspace.fire_alarm {});
          server = (rustPkgs.workspace.server {});
          temperature_gauge = (rustPkgs.workspace.temperature_gauge {});
          default = packages.server;
        };

        devShell = (rustPkgs.workspaceShell {
          packages = with pkgs; [ rust-analyzer ];
          shellHook = ''
            tmux new-session -d -t smarthouse-project-shell


            tmux split-window -h -t smarthouse-project-shell
            tmux resize-pane -t smarthouse-project-shell:0.1 -x 20%

            tmux send-keys -t smarthouse-project-shell:0.0 'hx' C-m

            # Docker-compose window
            tmux new-window -t smarthouse-project-shell

            tmux split-window -h -t smarthouse-project-shell:1
            tmux split-window -v -t smarthouse-project-shell:1.1

            tmux send-keys -t smarthouse-project-shell:1.0 'docker-compose up --attach server' C-m

            tmux send-keys -t smarthouse-project-shell:1.1 '
              while true; do
                docker-compose attach fire_alarm || { echo "Container has stopped. Reattaching..."; sleep 2; };
              done ' C-m

            tmux send-keys -t smarthouse-project-shell:1.2 '
              while true; do
                docker-compose attach temperature_gauge|| { echo "Container has stopped. Reattaching..."; sleep 2; };
              done ' C-m

            tmux attach-session -t smarthouse-project-shell

            while tmux has-session -t smarthouse-project-shell; do sleep 1; done
            exit
          '';
        });
      }
    );
}
