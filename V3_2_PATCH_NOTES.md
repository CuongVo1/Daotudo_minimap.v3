# V3.2 SOURCE PATCH — kiểm thử bắt buộc

- Thêm 123 POI Public vào chính Leaflet map (`src/livemap.js`), thay vì cố truy cập biến `map` của cửa sổ khác trong `v3.js`.
- POI là `L.marker` với `L.divIcon` phân loại Bunker/Abandoned/Secret/Police/Gas/Outpost/Other. Có lọc category, tìm kiếm và tooltip.
- POI neo theo tọa độ map 256 đơn vị và để Leaflet xử lý zoom/pan. Không dùng CSS left/top hay nhân zoom.
- Giữ player marker của V3.1, la bàn Overlay và sidebar AI.
- **Chưa hoàn tất:** Private POI chưa được đồng bộ an toàn từ Tauri launcher sang cửa sổ bản đồ riêng; không nhúng dữ liệu Private plaintext.
- **Chưa xác nhận:** phép biến đổi SCUM-MAP 0..320 sang tile-space 0..256 phải đối chiếu landmark thật; không khẳng định tọa độ chính xác chỉ từ mã.
- Chưa chạy Windows GUI / GitHub Actions trong môi trường này.
