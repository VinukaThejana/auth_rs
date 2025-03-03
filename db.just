set dotenv-load

connect:
  @echo "connecting to database"
  usql $(echo $DATABASE_URL)
