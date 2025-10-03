/* xkcd, Size: 305 Bytes */
static const uint16_t xkcd_first_encoding_table[20] = {
  65, 67, 68, 69, 70, 74, 75, 76, 79, 80, 81, 82, 83, 84, 85, 86, 87, 
  88, 89, 65535};
static const uint16_t xkcd_index_to_second_table[20] = {
  0, 1, 2, 3, 4, 17, 24, 26, 30, 33, 34, 45, 46, 48, 61, 62, 63, 
  64, 66, 71};
static const uint16_t xkcd_second_encoding_table[71] = {
  84, 86, 74, 86, 65, 66, 67, 69, 71, 72, 74, 77, 79, 81, 83, 85, 86, 
  67, 72, 74, 77, 81, 85, 86, 84, 86, 74, 84, 86, 89, 74, 84, 88, 
  74, 67, 71, 74, 76, 80, 81, 84, 85, 87, 88, 89, 74, 74, 84, 65, 
  66, 67, 69, 71, 72, 74, 77, 79, 81, 83, 85, 86, 74, 74, 74, 84, 
  86, 65, 67, 71, 74, 81};
static const uint8_t xkcd_kerning_values[71] = {
  2, 2, 2, 2, 2, 2, 3, 2, 2, 3, 4, 3, 2, 3, 2, 3, 3, 
  2, 3, 3, 3, 2, 3, 2, 2, 2, 3, 3, 2, 3, 2, 2, 2, 
  2, 2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 
  2, 3, 2, 2, 4, 4, 4, 2, 3, 2, 4, 3, 2, 2, 2, 2, 
  2, 2, 2, 2, 2, 2};
u8g2_kerning_t xkcd_k = {
  20, 71,
  xkcd_first_encoding_table,
  xkcd_index_to_second_table,
  xkcd_second_encoding_table,
  xkcd_kerning_values};

