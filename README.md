# 🌌 POLER-SciLang: Universal Scientific Scripting Language

> **Спеціалізована наукова мова програмування для Калькулятора Всього (Calculator of Everything).**  
> Створена для проведення одночасних розрахунків із десятків і сотень сфер науки (астрофізика, квантова механіка, органічна хімія, термодинаміка, релятивізм, фізика плазми, орбітальна механіка, чисельний аналіз).

---

## ⚡ Трієдина Архітектура (Tripartite Core)

1. **x86_64 Hand-Tuned AVX Assembly (`asm/avx2_kernels.s`):**
   - `asm_dot(a, b)`: Розгорнутий 256-бітний AVX скалярний добуток векторів на 8 паралельних потоків акумуляції.
   - `asm_poly(x, coeffs)`: Схема Горнера довільного степеня на регістрах FPU.
   - `asm_invsqrt(x)`: Апаратний `vrsqrtss` із двома кроками уточнення Ньютона-Рафсона подвійної точності IEEE 754.

2. **Zig 0.14.0 SIMD & Чисельні Інтегратори (`zig/matrix_simd.zig`):**
   - `matmul(A, B)` / `zig_matmul`: Кєш-оптимізоване блокове множення матриць через `@Vector(4, f64)`.
   - `norm(v)` / `zig_norm`: Швидка Евклідова $L_2$-норма.
   - `zig_fma(a, b, c)`: Векторний FMA $C_i += A_i \cdot B_i$.
   - `zig_rk4(y, t, dt, k, f)`: Чисельний інтегратор Рунге-Кутти 4-го порядку для ОДУ дифузії плазми та хімічної кінетики.

3. **Rust 1.98.1 Compiler & Dimensional Engine (`src/`):**
   - **7-вимірний тензорний контроль розмірностей SI:** $[L, M, T, I, \Theta, N, J]$. Фізичні одиниці (`m`, `kg`, `s`, `A`, `K`, `mol`, `N`, `J`, `W`, `Pa`, `V`, `ohm`, `Hz`, `km/h`, `ly`, `pc`, `M_sun`, `eV`, `g/mol` тощо) є частиною системи типів!
   - Автоматична перевірка узгодженості розмірностей та оператор перетворення `expr to <unit>`.
   - Інтерпретатор, REPL, підтримка пайплайнів `|>`, генерація виводу у `--json` для GUI.

---

## 🔬 Підтримувані Наукові Сфери

- **🌌 Астрофізика та ЗТВ:** `schwarzschild(M)`, `escape_velocity(M, r)`, `orbital_velocity(M, r)`, `kepler3_period(M, a)`, `grav_time_dilation(t, M, r)`, `roche_limit(R, rho_M, rho_m)`, `hubble_velocity(H0, d)`.
- **🚀 Релятивістська фізика:** `lorentz_gamma(v)`, `emc2(m)`, `rel_energy(m, v)`, `rel_momentum(m, v)`, `doppler_freq(f0, v)`.
- **⚛ Квантова та атомна фізика:** `photon_energy_lambda(lambda)`, `photon_energy(f)`, `de_broglie(p)`, `stefan_boltzmann(A, T)`, `wien_displacement(T)`, `hadamard()`, `pauli_x()`, `pauli_z()`.
- **🧪 Органічна хімія та біохімія:** `chem_molar_mass("C6H12O6")` (автоматичний парсер хімічних формул), кінетика `arrhenius(A, Ea, T)`, енергія Гіббса `gibbs_free_energy(dH, T, dS)`, рівняння Менделєєва-Клапейрона `ideal_gas_p(n, T, V)`.
- **⚡ Електродинаміка та плазма:** `coulomb_force(q1, q2, r)`, `magnetic_lorentz(q, v, B)`, час розряду `rc_tau(R, C)`, резонанс `lc_freq(L, C)`.
- **📐 Апаратне прискорення:** `asm_dot`, `asm_poly`, `asm_invsqrt`, `matmul`, `norm`, `zig_rk4`.

---

## 🛠 Збірка та Встановлення

Вимоги: `rustc/cargo` 1.80+, `zig` 0.13+, GNU `as` / `gcc`.

```bash
cd poler-scilang
cargo build --release
cp target/release/poler-scilang ~/.local/bin/
```

---

## 🚀 Використання

### 1. Інтерактивний REPL:
```bash
poler-scilang
```
```text
=== POLER-SciLang Interactive Scientific Engine (AVX2 + Zig + Rust) ===
scilang[Ψ] > let m = 1.0 kg;
scilang[Ψ] > emit m * c^2 to GJ;
=> 89875517.873682 GJ
scilang[Ψ] > chem_molar_mass("C8H10N4O2") to g/mol;
=> 194.194000 g/mol
```

### 2. Запуск файлу:
```bash
poler-scilang examples/star_collapse.psc
poler-scilang examples/quantum_biochem.psc
poler-scilang examples/fusion_reactor.psc
```

### 3. Однорядковий розрахунок:
```bash
poler-scilang -e "let M = 3.2 * M_sun; emit schwarzschild(M) to km;"
```

### 4. Режим JSON для GUI:
```bash
poler-scilang --json -e "let v = 100 km / 1 h; emit \"Швидкість\": v to m/s;"
```

---

## 📄 Ліцензія

POLER Open Sovereign License / MIT.
Частина екосистеми POLER Engine & Eteryya.
