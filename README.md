# casync-dashboard-backend
A web service for managing content for casync, desync, casync-rs type tools.
- Backend rust webserver and postgresql database

# Database setup
- Docker container of postgresql
- database name: diesel_demo
- table: chunks

# Web server
- To apis /chunks and /indexes are built
- Have to write apis for
-- Post index file with type(casync, desync, casync-rs)
-- Upload index file from UI
-- View List of indexes 
-- View chunks and sizes for each index
-- Chunk reusability stats
-- Track download count
