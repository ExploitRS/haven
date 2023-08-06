defmodule Haven.Application do
  @moduledoc false
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      {Plug.Cowboy, plug: Haven.Router, scheme: :http, options: [port: 4444]}
    ]

    opts = [strategy: :one_for_one, name: Haven.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
