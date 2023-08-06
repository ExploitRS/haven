defmodule Haven.Router do
  use Plug.Router

  plug :match
  plug :dispatch

  get "/" do
    conn
    |> put_resp_content_type("text/html")
    |> send_resp(200, "This is Haven in Exploit.RS")
  end

  match _ do
    send_resp(conn, 404, "Not found")
  end
end
