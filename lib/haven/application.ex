defmodule Haven.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Plug.Cowboy, scheme: :http, plug: Haven.Router, options: [port: 4000]}
    ]

    opts = [strategy: :one_for_one, name: Haven.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
